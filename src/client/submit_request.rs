use super::{auth::Auth, at_request::ATRequest, endpoints::Endpoints, errors::ClientError, Client};
use anyhow::Result;
use serde::Serialize;

impl Client {
    pub fn submit_post_request<T: Serialize + ?Sized>(
        &self,
        endpoint: Endpoints,
        body: Option<Box<T>>,
        auth: Option<Auth>,
    ) -> Result<reqwest::blocking::Response, ClientError> {
        let url = self.url(endpoint);

        let at_request = ATRequest {
            body: body,
            url: url.clone(),
            auth: auth.clone(),
        };

        let with_json = match &at_request.body {
            Some(json) => reqwest::blocking::Client::new()
                .post(url.clone())
                .json::<T>(json.as_ref())
                .header("Content-Type", "application/json"),
            None => reqwest::blocking::Client::new().post(url.clone()),
        };

        Ok(self.inject_authentication(&at_request, with_json).send()?)
    }

    fn inject_authentication<T: Serialize + ?Sized>(
        &self,
        at_request: &ATRequest<T>,
        request_builder: reqwest::blocking::RequestBuilder,
    ) -> reqwest::blocking::RequestBuilder {
        match &at_request.auth {
            Some(auth) => {
                let with_auth = match auth {
                    Auth::JWT(bearer_auth) => request_builder.bearer_auth(bearer_auth),
                    Auth::BasicAuth(basic_auth) => request_builder.basic_auth(
                        basic_auth.username.clone(),
                        Some(basic_auth.password.clone()),
                    ),
                };
                with_auth
            }
            None => request_builder,
        }
    }
}
