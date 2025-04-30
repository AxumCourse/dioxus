fn new_client() -> reqwest::Client {
    reqwest::Client::new()
}
pub fn new_post(url: &str) -> reqwest::RequestBuilder {
    let api_url = env!("API_URL");
    let url = format!("{}{}", api_url, url);

    new_client().post(&url)
}
