use super::endpoints::Endpoints;
use super::Client;
use serde::{Deserialize, Serialize};

impl Client {
    pub fn create_account(
        mut self,
        identifier: String,
        password: String,
        email: String,
        invite_code: String,
    ) -> reqwest::Result<AccountRecord> {
        let body = CreateAccount {
            handle: identifier,
            email: email,
            password: password,
            inviteCode: invite_code,
        };

        let response =
            self.submit_post_request(Endpoints::CreateAccount, Some(Box::new(body)), None)?;

        let account_record: AccountRecord = match response.json::<AccountRecord>() {
            Ok(json) => json,
            Err(e) => {
                println!("Account Creation Error: {}", e);
                return Err(e);
            }
        };
        self.jwt = account_record.accessJwt.to_owned();
        self.did = account_record.did.to_owned();
        Ok(account_record)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct CreateAccount {
    handle: String,
    email: String,
    password: String,
    inviteCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AccountRecord {
    pub handle: String,
    pub did: String,
    pub accessJwt: String,
    pub refreshJwt: String,
}
