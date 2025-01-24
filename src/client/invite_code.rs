use super::endpoints::Endpoints;
use super::Client;
use serde::{Deserialize, Serialize};

impl Client {
    pub fn create_invite_code(
        self,
        admin_username: String,
        admin_password: String,
        use_count: u32,
    ) -> reqwest::Result<InviteCodeRes> {
        let body = InviteCode {
            useCount: use_count,
        };
        let response = reqwest::blocking::Client::new()
            .post(self.url(Endpoints::CreateInviteCode))
            .header("Content-Type", "application/json")
            .json(&body)
            .basic_auth(admin_username, Some(admin_password))
            .send()?;

        response.json::<InviteCodeRes>()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct InviteCode {
    pub useCount: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InviteCodeRes {
    pub code: String,
}
