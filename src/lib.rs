#![feature(proc_macro_hygiene, decl_macro)]

use textnonce::TextNonce;

#[macro_use]
extern crate rocket;

pub mod guards;
pub mod routes;

pub struct ShopifyApp {
    pub access_mode: AccessMode,
    // pub auth_callback: Option<Box<dyn (Fn(Form<InstallCallbackQuery>) -> Redirect) + Send + Sync>>,
    pub credentials: Credentials,
    pub host: String,
    pub scopes: Vec<&'static str>,
}

impl ShopifyApp {
    // pub fn set_auth_callback<F>(&mut self, auth_callback: F)
    // where
    //     F: Fn(Form<InstallCallbackQuery>) -> Redirect + Send + Sync + 'static,
    // {
    //     self.auth_callback = Some(Box::new(auth_callback))
    // }

    pub fn install_uri(&self, shop: &str, return_uri: &str) -> (String, String) {
        let nonce = TextNonce::new().into_string();

        let redirect_uri = format!(
            "https://{shop}/admin/oauth/authorize?client_id={api_key}&scope={scopes}&redirect_uri={redirect_uri}&state={nonce}&grant_options[]={access_mode}",
            shop = shop,
            api_key = self.credentials.api_key,
            scopes=  self.scopes.join(","),
            redirect_uri = return_uri,
            nonce = nonce,
            access_mode = self.access_mode.as_string()
        );

        (redirect_uri, nonce)
    }
}

#[derive(Clone, Default)]
pub struct Credentials {
    pub api_key: String,
    pub secret: String,
}

impl Credentials {
    pub fn new(api_key: String, secret: String) -> Self {
        Credentials {
            api_key: api_key,
            secret: secret,
        }
    }
}

#[derive(Clone)]
pub enum AccessMode {
    Offline,
    Online,
}

impl AccessMode {
    pub fn as_string(&self) -> &'static str {
        match self {
            AccessMode::Offline => "offline",
            AccessMode::Online => "online",
        }
    }
}

impl Default for AccessMode {
    fn default() -> Self {
        AccessMode::Online
    }
}
