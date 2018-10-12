use url::Url;

use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::prelude::*;
use oauth2::{
  AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, RefreshToken, Scope,
  TokenUrl,
};

pub type Token = BasicTokenResponse;
use super::{Authorization, Error};

#[derive(Clone)]
pub enum Kind {
  OAuth2 {
    name: &'static str,
    auth_url: Url,
    token_url: Url,
  },
}

impl Kind {
  pub fn auth_url(&self) -> Url {
    use self::Kind::*;
    match self {
      OAuth2 { auth_url, .. } => auth_url.clone(),
      //_ => unreachable!(),
    }
  }

  pub fn token_url(&self) -> Url {
    use self::Kind::*;
    match self {
      OAuth2 { token_url, .. } => token_url.clone(),
      //_ => unreachable!(),
    }
  }

  pub fn name(&self) -> String {
    use self::Kind::*;
    match self {
      OAuth2 { name, .. } => name.to_string(),
      //_ => unreachable!(),
    }
  }
}

#[derive(Clone)]
pub struct Provider {
  client_id: ClientId,
  client_secret: ClientSecret,
  pub(crate) redirect_url: RedirectUrl,
  scopes: Vec<Scope>,
  provider: Kind,
  client: BasicClient,
}

impl Provider {
  pub fn authorization(&self) -> Authorization {
    let (url, state) = self.client.authorize_url(CsrfToken::new_random);
    Authorization { url, state }
  }

  pub fn token(&self, code: String) -> Result<Token, Error> {
    let code = AuthorizationCode::new(code);
    self
      .client
      .exchange_code(code)
      .map_err(|e| Error::OAuth2(e))
  }

  pub fn access_token(&self, refresh_token: &RefreshToken) -> Result<Token, Error> {
    self
      .client
      .exchange_refresh_token(refresh_token)
      .map_err(|e| Error::OAuth2(e))
  }

  pub fn eve_online() -> Kind {
    Kind::OAuth2 {
      name: "EveOnline",
      auth_url: Url::parse("https://login.eveonline.com/oauth/authorize").unwrap(),
      token_url: Url::parse("https://login.eveonline.com/oauth/token").unwrap(),
    }
  }

  pub fn redirect_url(&self) -> String {
    self.redirect_url.to_string()
  }

  pub fn name(&self) -> String {
    self.provider.name()
  }
}

pub struct ProviderOptions {
  id: Option<String>,
  secret: Option<String>,
  redirect_url: Option<String>,
  scopes: Vec<String>,
  provider: Option<Kind>,
}

impl ProviderOptions {
  pub fn new() -> Self {
    ProviderOptions {
      id: None,
      secret: None,
      redirect_url: None,
      scopes: vec![],
      provider: None,
    }
  }

  pub fn client_id(mut self, value: &str) -> Self {
    self.id = Some(value.to_string());
    self
  }

  pub fn client_secret(mut self, value: &str) -> Self {
    self.secret = Some(value.to_string());
    self
  }

  pub fn redirect_url(mut self, value: &str) -> Self {
    self.redirect_url = Some(value.to_string());
    self
  }

  pub fn scope(mut self, scope: &str) -> Self {
    self.scopes.push(scope.to_string());
    self
  }

  pub fn provider(mut self, provider: Kind) -> Self {
    self.provider = Some(provider);
    self
  }

  pub fn build(self) -> Result<Provider, Error> {
    let client_id = ClientId::new(self.id.ok_or(Error::MissingOption("client_id"))?);
    let client_secret =
      ClientSecret::new(self.secret.ok_or(Error::MissingOption("client_secret"))?);
    let redirect_url = RedirectUrl::new(Url::parse(
      &self
        .redirect_url
        .ok_or(Error::MissingOption("redirect_url"))?,
    )?);
    let provider = self.provider.ok_or(Error::MissingOption("provider"))?;
    let scopes = self
      .scopes
      .into_iter()
      .map(|scope| Scope::new(scope))
      .collect::<Vec<_>>();

    let mut client = BasicClient::new(
      client_id.clone(),
      Some(client_secret.clone()),
      AuthUrl::new(provider.auth_url()),
      Some(TokenUrl::new(provider.token_url())),
    )
    .set_redirect_url(redirect_url.clone());

    for scope in scopes.clone() {
      client = client.add_scope(scope);
    }

    Ok(Provider {
      client_id,
      client_secret,
      redirect_url,
      scopes,
      provider,
      client,
    })
  }
}
