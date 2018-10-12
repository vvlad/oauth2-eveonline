extern crate failure;
extern crate url;

use failure::Fail;
use oauth2::basic::BasicRequestTokenError;

mod authorization;
pub use self::authorization::Authorization;
mod provider;
pub use self::provider::{Provider, ProviderOptions, Token};

mod ui;
pub use self::ui::UI;

#[derive(Debug, Fail)]
pub enum Error {
  #[fail(display = "unknown error")]
  Unknown,

  #[fail(display = "Interrupted by user")]
  Interrupted,

  #[fail(display = "{}", _0)]
  OAuth2(#[cause] BasicRequestTokenError),

  #[fail(display = "missing option {}", _0)]
  MissingOption(&'static str),

  #[fail(display = "missing option {}", _0)]
  InvalidUrl(url::ParseError),

  #[fail(display = "invalid oauth state")]
  InvalidState,
}

impl From<url::ParseError> for Error {
  fn from(error: url::ParseError) -> Error {
    Error::InvalidUrl(error)
  }
}
