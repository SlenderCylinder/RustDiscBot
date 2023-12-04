#![allow(unused_imports)]
use serenity::{
    http::Http,
    async_trait, 
    client::{Client, Context, EventHandler}, 
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework, Args
    },
    model::prelude::*,
    model::id::UserId,
    model::gateway::Ready,
    model::channel::Message,
    builder::Timestamp,
    prelude::*,
    client::bridge::gateway::GatewayIntents,
};
use chrono::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use anyhow::anyhow;
use tokio;
use dotenv::dotenv;
use std::fmt;
use std::env;
use chrono::{FixedOffset, Duration};
use mongodb::{Client as MongoClient, options::ClientOptions}; // Use ::mongodb to refer to the crate
use mongodb::bson::Document;
use std::io::Cursor;
use crate::mongodb_connect::get_mongodb_client;

mod spotify;
mod openai;
mod mongodb_connect;
use crate::spotify::TrackInfo;

impl fmt::Display for TrackInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Track: {}\nArtists: {}\nAlbum: {}\nLink: {}\n",
            self.name, self.artists, self.album, self.link
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageData {
    username: String,
    content: String,
    timestamp: String,
}

#[group]
#[commands(hello, time, spomtify, rustgpt)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, _ctx: Context, msg: Message) {
        let user_id = msg.author.id; 
        let username = msg.author.name.clone();
        //let content = msg.content.clone();


        // list of allowed user IDs and keywords

        let allowed_users = vec![UserId(694679380068270170), UserId(490687063692279818)]; 
        //let allowed_keywords = vec!["keyword1", "keyword2"]; 


        // Check if the message is from an allowed user
        if allowed_users.contains(&user_id) {
            // Check if the message contains allowed keywords
            //if allowed_keywords.iter().any(|keyword| content.contains(keyword)) {
                let message_data = MessageData {
                    username: username.clone(),
                    content: msg.content.clone(),
                    timestamp: Utc::now().to_string(),
                };

        let serialized_data = serde_json::to_string(&message_data).expect("Serialization error");
        println!("Serialized Data: {}", serialized_data);
        
        // Push data to mongoDB
        let mongodb_client = mongodb_connect::get_mongodb_client().await.expect("Failed to obtain MongoDB client"); 
        let collection = mongodb_client.database("discord").collection("servermessages");

        let serialized_data_bytes = serialized_data.as_bytes();
        // Serialize the data structure directly to BSON
        let document = bson::to_document(&message_data).expect("Serialization to BSON error");
        match collection.insert_one(document, None).await {
            Ok(_) => {
                println!("Message inserted into MongoDB.");
            }
            Err(err) => {
                eprintln!("MongoDB insertion error: {:?}", err);
            }
        }

        println!("Received message from {}: {}", username, msg.content);
    }
}
    
    async fn presence_update(&self, ctx: Context, new_data: PresenceUpdateEvent) {
        // Get the user's ID from the presence update
        let user_id = new_data.presence.user_id;
        let http = ctx.http.clone();

        let username = match user_id.to_user(&http).await {
            Ok(user) => user.name,
            Err(_) => "Unknown".to_owned(), // Use "Unknown" as the default username if it couldn't be fetched
        };  
        // Get the presence status
        let status = new_data.presence.status;
        match status {
            OnlineStatus::Online => {
                if let Some(activity) = new_data.presence.activities.first() {
                    match &activity.kind {
                        ActivityType::Playing => {
                            if let Some(game) = &activity.details {
                                println!("User {} ({}) is {:?} and playing {}", username, user_id, status, game);
                            }
                        }
                        _ => {
                            println!("User {} ({}) is online", username, user_id);
                        }
                    }
                } else {
                    println!("User {} ({}) is online", username, user_id);
                }
            }
            OnlineStatus::Offline => {
                println!("User {} ({}) went offline",  username, user_id);
            }
            _ => {
                if let Some(activity) = new_data.presence.activities.first() {
                    match &activity.kind {
                        ActivityType::Playing => {
                            if let Some(game) = &activity.details {
                                println!("User {} ({}) is currently on {:?} mode and playing {}", username, user_id, status, game);
                            }
                        }
                        _ => {
                            println!("User {} ({}) changed presence status to {:?}", username, user_id, status);
                        }
                    }
                }
            }
        }
    }
    

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
        

}


#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new()
    .configure(|c| c.prefix("/"))
    .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");
    let intents = GatewayIntents::all();

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .intents(intents)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    } 
}



#[command]
async fn hello(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let now = Utc::now();
    let timestamp = now.to_rfc3339();
    let username = msg.author.name.clone();
    let response;
    if args.is_empty() {
        response = format!("Hi there, {}!", username);
    } else {
        let message = args.rest();
        response = format!("Hi there, {}! you said \"{}\"", username, message);    
    }

    let _msg = msg
    .channel_id
    .send_message(&ctx.http, |m| {
        m.content("Greetings!")
            .embed(|e| {
                e.title("YOOOOO!")
                    .description(response)
                    .image("https://media.tenor.com/bgE1zLqSAsYAAAAM/hey-wave.gif")
                    .fields(vec![
                        ("This is the first field", "This is a field body", true),
                        ("This is the second field", "Both fields are inline", true),
                    ])
                    .field("This is the third field", "This is not an inline field", false)
                    .footer(|f| f.text("This is a footer"))
                    .timestamp(timestamp)

            })
        })
        .await;

    Ok(())
}

#[command]
async fn time(ctx: &Context, msg: &Message) -> CommandResult {
    let username = msg.author.name.clone();
    let country = match username.as_str() {
        "slendercylinder" => "New York",
        _ => "Colombo",
    };
    let utc: DateTime<Utc> = Utc::now();
    let sl_time: DateTime<FixedOffset> = utc.with_timezone(&FixedOffset::east_opt(5 * 3600 + 1800).unwrap());    
    let offset = FixedOffset::west_opt(4 * 3600).unwrap(); // NY is 5 hours behind UTC
    let ny_time: DateTime<FixedOffset> = utc.with_timezone(&offset);

    let sl_time_str = sl_time.format("%I:%M %p").to_string();
    let sl_date_str = sl_time.format("%Y-%m-%d").to_string();

    let ny_time_str = ny_time.format("%I:%M %p").to_string();
    let ny_date_str = ny_time.format("%Y-%m-%d").to_string();

    let time = match username.as_str() {
        "slendercylinder" => ny_time_str,
        _ => sl_time_str,
    };

    let date = match username.as_str() {
        "slendercylinder" => ny_date_str,
        _ => sl_date_str,
    };

    let response = format!("Hi {}, the time in {} now is {} on {}.", username.as_str(), country, time, date);

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())

}



#[command]
async fn spomtify(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // Get the song name from the command arguments
    let song_name = args.rest();

    // Generate the bearer token
    let token_spot = spotify::generate_bearer_token().await.map_err(anyhow::Error::from)?;

    println!("Spotify Token Generated: {}", token_spot);

    let track_list = spotify::generate_track_list(&token_spot, song_name).await?;

    for track_info in track_list {
        let formatted_track_info = format!(
            "Track: {}\nArtists: {}\nAlbum: {}\nLink: {}\n",
            track_info.name, track_info.artists, track_info.album, track_info.link
        );
    
        msg.channel_id.say(&ctx.http, formatted_track_info).await?;
    }

    // Use the bearer token to generate the track list
    //spotify::generate_track_list(&token_spot).await?;


    Ok(())
}

#[command]
async fn rustgpt(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    dotenv().ok();
    // Get the message sent by the user
    let message = args.rest();
    
    // Call the function to chat with GPT-3
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let gpt_response = openai::chat_with_gpt(message, &api_key).await.map_err(anyhow::Error::from)?;
    
    // Send the GPT-3 response as a message
    msg.channel_id.say(&ctx.http, gpt_response).await?;
    
    Ok(())
}

/*#[command]
async fn clone(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    let username = msg.author.name.clone();
    let useravatar = msg.author;

}*/

