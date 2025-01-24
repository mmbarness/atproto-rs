use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Error making request: `{0}`")]
    RequestError(#[from] reqwest::Error),
    #[error("Invalid authentication credentials")]
    AuthenticationError,
}

impl From<ClientError> for reqwest::Error {
    fn from(value: ClientError) -> Self {
        match value {
            ClientError::RequestError(e) => e,
            ClientError::AuthenticationError => todo!(),
        }
    }
}
