mod event_handler;
mod news_thread;
mod news_command;
mod bot_config;

use matrix_sdk::{
    config::SyncSettings
    ,
    Client,
};

use crate::bot_config::{parse_config, NewBotConfig};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_path = std::env::args().nth(1);
    let bot_config = parse_config(config_path);

    // our actual runner
    login_and_sync(bot_config).await?;
    Ok(())
}

// The core sync loop we have running.
async fn login_and_sync(
    bot_config: NewBotConfig
) -> anyhow::Result<()> {
    let client = Client::builder()
        .homeserver_url(&bot_config.matrix_homerserver)
        .build()
        .await?;

    client
        .matrix_auth()
        .login_username(&bot_config.matrix_username, &bot_config.matrix_password)
        .initial_device_display_name(&bot_config.bot_name)
        .await?;

    println!("logged in as {}", &bot_config.matrix_username);

    client.add_event_handler(event_handler::on_stripped_state_member);
    let sync_token = client.sync_once(SyncSettings::default()).await?.next_batch;
    client.add_event_handler(event_handler::on_room_message);
    news_thread::start(&client, &bot_config).await;

    loop {
        let settings = SyncSettings::default().token(&sync_token);
        let _ = client.sync(settings).await; // this essentially loops until we kill the bot
        eprintln!("Error on sync! \nReconnecting in 4 min...");
        sleep(Duration::from_secs(60 * 4)).await;
    }
}