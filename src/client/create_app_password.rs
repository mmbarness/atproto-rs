use super::endpoints::Endpoints;
use super::Client;
use serde::{Deserialize, Serialize};

impl Client {
    pub fn create_app_password(&mut self, name: String) -> reqwest::Result<AppSpecificPasswordRes> {
        let body = AppSpecificPassword { name: name };
        let response =
            self.submit_post_request(Endpoints::RefreshSession, Some(Box::new(body)), None)?;

        match response.json::<AppSpecificPasswordRes>() {
            Ok(json) => Ok(json),
            Err(e) => Err(e),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AppSpecificPassword {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AppSpecificPasswordRes {
    pub handle: String,
    pub did: String,
    pub accessJwt: String,
    pub refreshJwt: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AppPassword {
    pub name: String,
    pub password: String,
    pub createdAt: String,
}
