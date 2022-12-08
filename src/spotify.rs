/*
Token can be found here: 
https://developer.spotify.com/console/get-search-item/?q=Muse&type=track&market=US&limit=5&offset=5&include_external=
*/
use reqwest;
use http::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, "Bearer <token>".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    let url = format!("https://api.spotify.com/v1/search?q={query}]&type=track,artist", query = "Gorillaz");

    let client = reqwest::Client::new();
    let res = client.get(&url)
        .headers(headers)
        .send()
        .await
        .unwrap()
        .text()
        .await;
    println!("{:?}", res);
}