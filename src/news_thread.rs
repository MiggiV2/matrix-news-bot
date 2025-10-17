use crate::bot_config::NewBotConfig;
use crate::news_command;
use chrono::{Local, NaiveTime};
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use matrix_sdk::ruma::RoomId;
use matrix_sdk::Client;
use std::time::Duration;
use tokio::time::sleep;

pub async fn start(client: &Client, new_bot_config: &NewBotConfig) {
    let room_id = RoomId::parse(&new_bot_config.matrix_room_id)
        .expect("Can't parse room!");
    let room = client.get_room(&room_id)
        .expect("Failed to get room!");

    let target_time = NaiveTime::parse_from_str(&new_bot_config.news_time, "%H:%M")
        .expect("Failed to parse news_time from config. Expected format: HH:MM");
    let update_frequency_minutes = parse_duration(&new_bot_config.update_frequency)
        .expect("Failed to parse update_frequency from config. Expected format: e.g., '24h', '30m'");
    let mut time_till_news = minutes_until(target_time);
    let minutes = time_till_news % 60;
    let hours = (time_till_news - minutes) / 60;

    let msg = format!("⏰ Sending news in {} hours {} minutes...", hours, minutes);
    let content = RoomMessageEventContent::text_plain(msg);
    if let Err(e) = room.send(content).await {
        eprintln!("Failed to send message! {}", e);
    }

    // new thread
    tokio::spawn(async move {
        loop {
            println!("Sleeping for {} minutes...", time_till_news);
            sleep(Duration::from_secs(time_till_news * 60)).await;
            let news_msg = news_command::build_news_msg().await;
            if let Err(e) = room.send(news_msg).await {
                eprintln!("Failed to send message! {}", e);
            }
            time_till_news = update_frequency_minutes;
        }
    });
}

fn minutes_until(target_time: NaiveTime) -> u64 {
    let now = Local::now().time();

    let duration = if now < target_time {
        target_time - now
    } else {
        let until_midnight = NaiveTime::from_hms_opt(23, 59, 59).unwrap() - now;
        let midnight_until_target = target_time - NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        until_midnight + midnight_until_target
    };

    duration.num_minutes() as u64
}

fn parse_duration(duration_str: &str) -> Result<u64, String> {
    let duration_str = duration_str.trim();
    
    if duration_str.ends_with('h') {
        let hours = duration_str[..duration_str.len() - 1]
            .parse::<u64>()
            .map_err(|_| format!("Invalid hours value in '{}'", duration_str))?;
        Ok(hours * 60)
    } else if duration_str.ends_with('m') {
        let minutes = duration_str[..duration_str.len() - 1]
            .parse::<u64>()
            .map_err(|_| format!("Invalid minutes value in '{}'", duration_str))?;
        Ok(minutes)
    } else {
        Err(format!("Duration must end with 'h' (hours) or 'm' (minutes), got: '{}'", duration_str))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveTime;

    #[test]
    fn test_minutes_until() {
        let target = NaiveTime::from_hms_opt(6, 0, 0).unwrap();
        let duration = minutes_until(target);

        let minutes = duration % 60;
        let hours = (duration - minutes) / 60;
        println!("{}h {}m", hours, minutes);
    }
}
