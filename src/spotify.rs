/*
Token can be found here: 
https://developer.spotify.com/console/get-search-item/?q=Muse&type=track&market=US&limit=5&offset=5&include_external=
*/
use reqwest::{Client, StatusCode};
use http::{header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE}};
use serde::{Deserialize, Serialize};
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: <searchstring>");
        std::process::exit(0);
    }
    let searchstring = &args[1];
    let spotify_api_key = match env::var("SPOTIFY_API_KEY") {
        Ok(value) => value,
        Err(_e) => {
            eprintln!("Error, you need to set the env variable: SPOTIFY_API_KEY (ie export SPOTIFY_API_KEY=123456789\nToken can be found here:\nhttps://developer.spotify.com/console/get-search-item/?q=Muse&type=track&market=US&limit=5&offset=5&include_external=");
            return;
        }
    };
    let bearer = format!("Bearer {}", spotify_api_key);
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, bearer.parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    let url = format!("https://api.spotify.com/v1/search?q={query}]&type=track,artist", query=searchstring);

    let client = Client::new();
    let res = client.get(&url)
        .headers(headers)
        .send()
        .await
        .unwrap();
    
    match res.status() {
        StatusCode::OK => {
            match res.json::<APIResponse>().await {
                Ok(parsed) => {
                    println!("{:?}", print_tracks(parsed.tracks.items));
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }

        },
        StatusCode::UNAUTHORIZED => {
            println!("You need to grab a new token: {}\nToken can be found here:\nhttps://developer.spotify.com/console/get-search-item/?q=Muse&type=track&market=US&limit=5&offset=5&include_external=", res.status());
        },
        _ => {
            println!("Fuckery is going on..(Error): {}", res.status());
        }
        
    }
    
}

fn print_tracks(tracks: Vec<Track>) {
    for track in tracks {
        println!("Track: {}", track.name);
        println!("Album: {}", track.album.name);
        println!("Artist: {}", track.album.artists[0].name);
        println!("Popularity: {}", track.popularity);
        println!("\tSpotify Link: {}", track.external_urls.spotify);
        println!("----");
    }
}

#[derive(Serialize, Deserialize)]
struct ExternalUrls {
    spotify: String,
}
#[derive(Serialize, Deserialize)]
struct Artist {
    name: String,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize)]
struct Items<T> {
    items: Vec<T>,
}
#[derive(Serialize, Deserialize)]
struct APIResponse {
    tracks: Items<Track>,
}
#[derive(Serialize, Deserialize)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}