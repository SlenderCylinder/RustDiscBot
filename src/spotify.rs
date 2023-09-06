use reqwest::Client;
use base64;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
struct SpotifyResponse {
    tracks: TrackList,
}

#[derive(Debug, Serialize, Deserialize)]
struct TrackList {
    items: Vec<Track>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Track {
    id: String,
    name: String,
    artists: Vec<Artist>,
    album: Album,
}

#[derive(Debug, Serialize, Deserialize)]
struct Artist {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Album {
    name: String,
}

pub async fn generate_bearer_token() -> Result<String> {
    // Spotify API credentials
    const CLIENT_ID: &str = "8bf89ef32a4241a0977c4968fc3b9b5a";
    const CLIENT_SECRET: &str = "c10a247c1ae14edd858d49dbfa7457da";
    const REDIRECT_URI: &str = "https://slendercylinder.me";

    // URLS
    const AUTH_URL: &str = "https://accounts.spotify.com/authorize";
    const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";

    // Make a request to the /authorize endpoint to get an authorization code
    let auth_url = format!(
        "{}?client_id={}&response_type=code&redirect_uri={}&scope=playlist-modify-private",
        AUTH_URL, CLIENT_ID, REDIRECT_URI
    );
    let auth_code = reqwest::get(&auth_url)
        .await?
        .text()
        .await?;

    let auth_header = base64::encode(format!("{}:{}", CLIENT_ID, CLIENT_SECRET));
    let client = Client::new();
    let token_payload = [
        ("grant_type", "client_credentials"),
        ("code", &auth_code),
        ("redirect_uri", REDIRECT_URI),
    ];
    let access_token_request = client
        .post(TOKEN_URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Authorization", format!("Basic {}", auth_header))
        .form(&token_payload)
        .send()
        .await?;

    // Convert the response to JSON
    let access_token_response_data: serde_json::Value = access_token_request
        .json()
        .await?;

    let access_token = access_token_response_data["access_token"]
        .as_str()
        .ok_or_else(|| anyhow::Error::msg("Failed to get access token"))?
        .to_owned();

    Ok(access_token)
}

#[derive(Clone)]
pub struct TrackInfo {
    pub name: String,
    pub artists: String,
    pub album: String,
    pub link: String,
}


pub async fn generate_track_list(token: &str, song: &str) -> Result<Vec<TrackInfo>> {
    // Prompt the user to enter a search query
    let input = song;
    //std::io::stdin().read_line(&mut input)?;
    //let query = input.trim();
    // Build the request URL
    let url = format!(
        "https://api.spotify.com/v1/search?q={}&type=track&limit=5",
        input
    );
    // Send the request to Spotify API
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    // Parse the response JSON
    let response_json: SpotifyResponse = response.json().await?;
    let mut track_info_list: Vec<TrackInfo> = Vec::new();
for track in response_json.tracks.items {
    let artists = track
        .artists
        .iter()
        .map(|artist| artist.name.clone())
        .collect::<Vec<String>>()
        .join(", ");

    // Create a TrackInfo struct or data structure to hold the track information
    let track_info = TrackInfo {
        name: track.name.clone(),
        artists: artists.clone(),
        album: track.album.name.clone(),
        link: format!("https://open.spotify.com/track/{}", track.id),
    };

    // Add the cloned track info to the list
    track_info_list.push(track_info.clone());

    // Print the track information
    /*println!("Track: {}", track.name);
    println!("Artists: {}", artists);
    println!("Album: {}", track.album.name);
    println!("Link: {}", track_info.link);
    println!();*/
}

    Ok(track_info_list)
}
