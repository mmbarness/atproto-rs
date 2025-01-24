use super::endpoints::Endpoints;
use super::Client;
use serde::{Deserialize, Serialize};

impl Client {
    pub fn resolve_handle(&self, handle: String) -> reqwest::Result<String> {
        let body = ResolveHandle { handle: handle };
        let response =
            self.submit_post_request(Endpoints::ResolveHandle, Some(Box::new(body)), None)?;

        match response.json::<ResolveHandleRes>() {
            Ok(json) => Ok(json.did),
            Err(e) => return Err(e),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResolveHandle {
    pub handle: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResolveHandleRes {
    pub did: String,
}
