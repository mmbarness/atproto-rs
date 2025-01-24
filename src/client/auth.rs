use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum Auth {
    BasicAuth(BasicAuth),
    JWT(JWT),
}

type JWT = String;

#[derive(Clone, Serialize)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    pub fn new(username: &String, password: &String) -> Self {
        Self {
            username: username.clone(),
            password: password.clone(),
        }
    }
}
