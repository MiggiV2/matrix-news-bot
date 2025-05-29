use chrono::{Local, NaiveTime, Utc};
use matrix_sdk::ruma::RoomId;
use matrix_sdk::{
    config::SyncSettings,
    ruma::events::room::{
        member::StrippedRoomMemberEvent,
        message::{MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent},
    },
    Client, Room, RoomState,
};
///
///  This is an example showcasing how to build a very simple bot using the
/// matrix-sdk. To try it, you need a rust build setup, then you can run:
/// `cargo run -p example-getting-started -- <homeserver_url> <user> <password>`
///
/// Use a second client to open a DM to your bot or invite them into some room.
/// You should see it automatically join. Then post `!party` to see the client
/// in action.
///
/// Below the code has a lot of inline documentation to help you understand the
/// various parts and what they do
// The imports we need
use std::{env, process::exit};
use tagesschau_lib::{News, Tagesschau};
use tokio::time::{sleep, Duration};

/// This is the starting point of the app. `main` is called by rust binaries to
/// run the program in this case, we use tokio (a reactor) to allow us to use
/// an `async` function run.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // set up some simple stderr logging. You can configure it by changing the env
    // var `RUST_LOG`
    tracing_subscriber::fmt::init();

    // parse the command line for homeserver, username and password
    let (homeserver_url, username, password) =
        match (env::args().nth(1), env::args().nth(2), env::args().nth(3)) {
            (Some(a), Some(b), Some(c)) => (a, b, c),
            _ => {
                eprintln!(
                    "Usage: {} <homeserver_url> <username> <password>",
                    env::args().next().unwrap()
                );
                // exit if missing
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
    // First, we set up the client.

    // Note that when encryption is enabled, you should use a persistent store to be
    // able to restore the session with a working encryption setup.
    // See the `persist_session` example.
    let client = Client::builder()
        // We use the convenient client builder to set our custom homeserver URL on it.
        .homeserver_url(homeserver_url)
        .build()
        .await?;

    // Then let's log that client in
    client
        .matrix_auth()
        .login_username(username, password)
        .initial_device_display_name("Tagesschau News")
        .await?;

    // It worked!
    println!("logged in as {username}");

    // Now, we want our client to react to invites. Invites sent us stripped member
    // state events so we want to react to them. We add the event handler before
    // the sync, so this happens also for older messages. All rooms we've
    // already entered won't have stripped states anymore and thus won't fire
    client.add_event_handler(on_stripped_state_member);

    // An initial sync to set up state and so our bot doesn't respond to old
    // messages. If the `StateStore` finds saved state in the location given the
    // initial sync will be skipped in favor of loading state from the store
    let sync_token = client.sync_once(SyncSettings::default()).await.unwrap().next_batch;

    // now that we've synced, let's attach a handler for incoming room messages, so
    // we can react on it
    client.add_event_handler(on_room_message);

    // Add loop here
    start_news_thread(&client).await;

    // since we called `sync_once` before we entered our sync loop we must pass
    // that sync token to `sync`
    let settings = SyncSettings::default().token(sync_token);
    // this keeps state from the server streaming in to the bot via the
    // EventHandler trait
    client.sync(settings).await?; // this essentially loops until we kill the bot

    Ok(())
}

// Whenever we see a new stripped room member event, we've asked our client to
// call this function. So what exactly are we doing then?
async fn on_stripped_state_member(
    room_member: StrippedRoomMemberEvent,
    client: Client,
    room: Room,
) {
    if room_member.state_key != client.user_id().unwrap() {
        // the invite we've seen isn't for us, but for someone else. ignore
        return;
    }

    // The event handlers are called before the next sync begins, but
    // methods that change the state of a room (joining, leaving a room)
    // wait for the sync to return the new room state so we need to spawn
    // a new task for them.
    tokio::spawn(async move {
        println!("Autojoining room {}", room.room_id());
        let mut delay = 2;

        while let Err(err) = room.join().await {
            // retry autojoin due to synapse sending invites, before the
            // invited user can join for more information see
            // https://github.com/matrix-org/synapse/issues/4345
            eprintln!("Failed to join room {} ({err:?}), retrying in {delay}s", room.room_id());

            sleep(Duration::from_secs(delay)).await;
            delay *= 2;

            if delay > 3600 {
                eprintln!("Can't join room {} ({err:?})", room.room_id());
                break;
            }
        }
        println!("Successfully joined room {}", room.room_id());
    });
}

// This fn is called whenever we see a new room message event. You notice that
// the difference between this and the other function that we've given to the
// handler lies only in their input parameters. However, that is enough for the
// rust-sdk to figure out which one to call and only do so, when the parameters
// are available.
async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
    // First, we need to unpack the message: We only want messages from rooms we are
    // still in and that are regular text messages - ignoring everything else.
    if room.state() != RoomState::Joined {
        return;
    }
    let MessageType::Text(text_content) = event.content.msgtype else { return };

    // here comes the actual "logic": when the bot see's a `!party` in the message,
    // it responds
    if text_content.body.contains("!party") {
        let content = RoomMessageEventContent::text_plain("🎉🎊🥳 let's PARTY!! 🥳🎊🎉");

        println!("sending");

        // send our message to the room we found the "!party" command in
        room.send(content).await.unwrap();

        println!("message sent");
    }
    if text_content.body == "!news" {
        let content = build_news_msg().await;
        room.send(content).await.unwrap();
    }
}

async fn start_news_thread(client: &Client) {
    let room_id = RoomId::parse("!hFekksusgjPusUvBbO:matrix.familyhainz.de")
        .expect("Can't parse room!");
    let room = client.get_room(&room_id)
        .expect("Failed to get room!");

    let mut time_till_news = minutes_until(NaiveTime::from_hms_opt(6, 0, 0).unwrap());
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
            let news_msg = build_news_msg().await;
            if let Err(e) = room.send(news_msg).await {
                eprintln!("Failed to send message! {}", e);
            }
            time_till_news = 24 * 60 * 60;
        }
    });
}
async fn build_news_msg() -> RoomMessageEventContent {
    let result = tokio::task::spawn_blocking(move || {
        let mut tagesschau = Tagesschau::new();
        tagesschau.fetch()
    }).await;

    let content = match result {
        Ok(Ok(news)) => {
            let now = Utc::now();
            let formatted_date = now.format("%d.%m.%Y");
            let content = format!(
                "🌐 Top 3 News Today | {}\n\n{}", // Add extra line break after the header
                formatted_date,
                format!(
                    "1. {}\n\n2. {}\n\n3. {}\n",
                    print_news(news.news.get(0).unwrap()),
                    print_news(news.news.get(1).unwrap()),
                    print_news(news.news.get(2).unwrap())
                )
            );
            RoomMessageEventContent::text_plain(content)
        }
        _ => RoomMessageEventContent::text_plain("Error!"),
    };
    content
}

fn print_news(news: &News) -> String {
    let source = news.share_url.clone();
    let tags = news.tags.iter().map(|t| t.tag.clone()).collect::<Vec<_>>().join(", ");
    format!("{}\n{}\nTags: {}\nSource: {}", &news.title, &news.first_sentence.clone().unwrap_or_default(), tags, source.unwrap_or_default())
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
