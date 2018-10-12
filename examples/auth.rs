extern crate oauth2_eveonline;

use oauth2_eveonline::{Provider, ProviderOptions, UI};

fn main() {
  let provider = ProviderOptions::new()
    .client_id("redacted")
    .client_secret("redacted")
    .redirect_url("eveauth-app://eintel")
    .scope("publicData")
    .provider(Provider::eve_online())
    .build()
    .unwrap();

  let ui = UI::new(provider.clone());

  let token = ui.token().unwrap();

  println!("{:?}", token);

  let access_token = provider.access_token(&token.refresh_token().unwrap());
  println!("{:?}", access_token);
}
