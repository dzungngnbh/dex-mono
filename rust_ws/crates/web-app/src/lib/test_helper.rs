use reqwest::Client;

// client for integration testing
pub fn get_https_client() -> Client {
    Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
}
