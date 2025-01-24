use super::endpoints::Endpoints;
use super::auth::Auth;
use super::Client;
use serde::{Deserialize, Serialize};

impl Client {
    pub fn refresh(&mut self) -> reqwest::Result<RefreshSessionRes> {
        let response = self.submit_post_request::<Empty>(
            Endpoints::RefreshSession,
            None,
            Some(Auth::JWT(self.jwt.clone())),
        )?;

        let res_json = match response.json::<RefreshSessionRes>() {
            Ok(json) => json,
            Err(e) => {
                return Err(e);
            }
        };

        self.jwt = res_json.accessJwt.to_owned();
        Ok(res_json)
    }
}

#[derive(Serialize)]
struct Empty {}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct RefreshSessionRes {
    pub accessJwt: String,
    pub refreshJwt: String,
    pub handle: String,
    pub did: String,
}
