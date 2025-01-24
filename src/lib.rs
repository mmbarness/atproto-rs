pub mod client;
pub use client::Client;

impl Default for Client {
    fn default() -> Self {
        Self {
            identifier: "".to_string(),
            base_url: "https://bsky.social/".to_string(),
            did: "".to_string(),
            jwt: "".to_string(),
            refresh_jwt: "".to_string(),
        }
    }
}
