use super::Client;
use super::{endpoints::Endpoints, auth::{Auth, BasicAuth}};
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

        let response = self.submit_post_request(
            Endpoints::CreateInviteCode,
            Some(Box::new(body)),
            Some(Auth::BasicAuth(BasicAuth::new(
                &admin_username,
                &admin_password,
            ))),
        )?;

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
