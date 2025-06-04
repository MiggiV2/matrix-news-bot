mod event_handler;
mod news_thread;
mod news_command;

use matrix_sdk::{
    config::SyncSettings
    ,
    Client,
};

use std::{env, process::exit};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (homeserver_url, username, password) =
        match (env::args().nth(1), env::args().nth(2), env::args().nth(3)) {
            (Some(a), Some(b), Some(c)) => (a, b, c),
            _ => {
                eprintln!(
                    "Usage: {} <homeserver_url> <username> <password>",
                    env::args().next().unwrap()
                );
                exit(1)
            }
        };

    // our actual runner
    login_and_sync(homeserver_url, &username, &password).await?;
    Ok(())
}

// The core sync loop we have running.
async fn login_and_sync(
    homeserver_url: String,
    username: &str,
    password: &str,
) -> anyhow::Result<()> {
    let client = Client::builder()
        .homeserver_url(homeserver_url)
        .build()
        .await?;

    client
        .matrix_auth()
        .login_username(username, password)
        .initial_device_display_name("Tagesschau News")
        .await?;

    println!("logged in as {username}");

    client.add_event_handler(event_handler::on_stripped_state_member);
    let sync_token = client.sync_once(SyncSettings::default()).await?.next_batch;
    client.add_event_handler(event_handler::on_room_message);
    news_thread::start(&client).await;

    let settings = SyncSettings::default().token(&sync_token);
    client.sync(settings).await?; // this essentially loops until we kill the bot

    Ok(())
}