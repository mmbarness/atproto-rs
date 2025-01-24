use super::{at_request::ATRequest, endpoints::Endpoints, Client};
use serde::Serialize;

impl Client {
    pub fn submit_post_request<T: Serialize + ?Sized>(
        &self,
        endpoint: Endpoints,
        body: Option<Box<T>>,
        jwt: Option<String>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let url = self.url(endpoint);

        let at_request = ATRequest {
            body: body,
            url: url,
            jwt: jwt,
        };

        let request = reqwest::blocking::Client::new()
            .post(at_request.url)
            .header("Content-Type", "application/json");

        let with_json = match at_request.body {
            Some(json) => {
                let cloned = json.as_ref();
                request.json::<T>(cloned)
            }
            None => request,
        };

        let with_bearer_auth = match at_request.jwt {
            Some(bearer_auth) => with_json.bearer_auth(bearer_auth),
            None => with_json,
        };

        with_bearer_auth.send()
    }
}
