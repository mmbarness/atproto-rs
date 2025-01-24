use serde::Serialize;

#[derive(serde::Serialize, Clone)]
pub struct ATRequest<SerializablePayload: Serialize + ?Sized> {
    pub body: Option<Box<SerializablePayload>>,
    pub jwt: Option<String>,
    pub url: String
}
