use oauth2::CsrfToken;
use url::Url;

pub struct Authorization {
  pub(crate) state: CsrfToken,
  pub(crate) url: Url,
}

impl Authorization {
  pub fn url(&self) -> String {
    format!("{}", self.url)
  }
}
