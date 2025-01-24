use super::endpoints::Endpoints;
use super::auth::Auth;
use super::Client;
use chrono::Utc;
use serde::{Deserialize, Serialize};

impl Client {
    pub fn post(&self, did: String, jwt: String, post_text: String) -> reqwest::Result<PostRes> {
        let now = Utc::now().to_rfc3339().to_string();
        let body = Post {
            repo: did,
            collection: "app.bsky.feed.post".to_string(),
            record: Record {
                _type: "app.bsky.feed.post".to_string(),
                text: post_text.to_owned(),
                createdAt: now,
            },
        };

        let response = self.submit_post_request(
            Endpoints::CreateRecord,
            Some(Box::new(body)),
            Some(Auth::JWT(jwt)),
        )?;

        match response.json::<PostRes>() {
            Ok(json) => Ok(json),
            Err(e) => Err(e),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostRes {
    pub uri: String,
    pub cid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub repo: String,
    pub collection: String,
    pub record: Record,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Record {
    pub _type: String,
    pub text: String,
    pub createdAt: String,
}
