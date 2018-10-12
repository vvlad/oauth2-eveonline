extern crate glib;
extern crate gtk;

extern crate webkit2gtk;

use gtk::prelude::*;
use gtk::{ContainerExt, Inhibit, WidgetExt, Window, WindowType};
use oauth2::prelude::*;
use url::Url;
use webkit2gtk::{SettingsExt, WebContext, WebView, WebViewExt};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::{Authorization, Error, Provider, Token};

pub struct UI {
  provider: Provider,
}

impl UI {
  pub fn new(provider: Provider) -> Self {
    UI { provider }
  }

  pub fn token(&self) -> Result<Token, Error> {
    let authorization = self.provider.authorization();
    let url = self.token_url(&authorization).ok_or(Error::Interrupted)?;
    let params = decode_url(url)?;
    let code = params.get("code").ok_or(Error::MissingOption("code"))?;
    let state = params.get("state").ok_or(Error::MissingOption("state"))?;

    if state != authorization.state.secret() {
      Err(Error::InvalidState)
    } else {
      self.provider.token(code.to_string())
    }
  }

  fn token_url(&self, authorization: &Authorization) -> Option<String> {
    let url = authorization.url();
    let redirect_url = self.provider.redirect_url();
    let token_url: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    gtk::init().unwrap();

    let window = Window::new(WindowType::Toplevel);
    let context = WebContext::get_default().unwrap();

    let web_view = WebView::new_with_context(&context);
    web_view.load_uri(&url);

    {
      let token_url = token_url.clone();
      web_view.connect_property_uri_notify(move |web_view| {
        if let Some(uri) = web_view.get_uri() {
          if uri.starts_with(&redirect_url.clone()) {
            token_url.replace(Some(uri));
            gtk::main_quit();
          }
        };
      });
    }
    window.add(&web_view);
    window.set_size_request(450, 700);
    window.set_title(&self.provider.name());

    let settings = WebViewExt::get_settings(&web_view).unwrap();
    settings.set_enable_developer_extras(true);

    window.show_all();

    window.connect_delete_event(|_, _| {
      gtk::main_quit();
      Inhibit(false)
    });

    gtk::main();
    let url = token_url.borrow().clone();
    url
  }
}

fn decode_url(url: String) -> Result<HashMap<String, String>, Error> {
  let mut params = HashMap::new();
  let url = Url::parse(&url)?;

  for (name, value) in url.query_pairs() {
    params.insert(name.to_string(), value.to_string());
  }

  Ok(params)
}
