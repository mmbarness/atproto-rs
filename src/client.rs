use derive_getters::Getters;
mod at_request;
mod auth;
mod create_account;
mod create_app_password;
mod endpoints;
mod errors;
mod invite_code;
mod login;
mod post;
mod refresh;
mod resolve_handle;
mod submit_request;

#[derive(Getters)]
pub struct Client {
    pub identifier: String,
    pub base_url: String,
    pub did: String,
    pub jwt: String,
    pub refresh_jwt: String,
}

impl Client {
    pub fn new(base_url: &String) -> Client {
        Self {
            base_url: base_url.to_string(),
            ..Default::default()
        }
    }

    pub fn url(&self, ext: endpoints::Endpoints) -> String {
        "".to_string() + &self.base_url.clone() + "xrpc/" + &ext.to_string()
    }
}
