use super::auth::Auth;
use serde::Serialize;

#[derive(serde::Serialize, Clone)]
pub struct ATRequest<SerializablePayload: Serialize + ?Sized> {
    pub auth: Option<Auth>,
    pub body: Option<Box<SerializablePayload>>,
    pub url: String,
}
