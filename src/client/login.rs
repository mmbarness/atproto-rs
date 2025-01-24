use super::endpoints::Endpoints;
use super::Client;
use serde::{Deserialize, Serialize};

impl Client {
    pub fn login(&mut self, identifier: &String, password: String) -> reqwest::Result<LoginRes> {
        let body = Login {
            identifier: identifier.to_owned(),
            password: password,
        };

        let response =
            self.submit_post_request(Endpoints::CreateSession, Some(Box::new(body)), None)?;

        let serialized_response = match response.json::<LoginRes>() {
            Ok(json) => json,
            Err(e) => {
                println!("Login Error: {}", e);
                return Err(e);
            }
        };

        self.jwt = serialized_response.accessJwt.to_owned();
        self.refresh_jwt = serialized_response.refreshJwt.to_owned();
        self.did = serialized_response.did.to_owned();
        Ok(serialized_response)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct LoginRes {
    pub did: String,
    pub handle: String,
    pub email: String,
    pub accessJwt: String,
    pub refreshJwt: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub identifier: String,
    pub password: String,
}
