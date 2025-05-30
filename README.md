# Tagesschau Matrix Bot

A simple Rust-powered Matrix bot that sends you the latest news from Germany's leading news portal [Tagesschau](https://www.tagesschau.de/).

## Features

- **Automated News Updates**: The bot posts the top 3 news stories from Tagesschau every morning at 6:00 AM (local time) to your chosen Matrix room.
- **On-Demand News**: Send the command `!news` in any room the bot is in, and it will respond with the latest headlines.
- **Auto-Join**: The bot will automatically join rooms it is invited to.
- **Fun Command**: Send `!party` and get a celebratory response!

## Usage

### Prerequisites

- Rust toolchain (install via [rustup.rs](https://rustup.rs/))
- A Matrix account and access to a homeserver
- The [tagesschau_lib crate](https://code.mymiggi.de/Miggi/tagesschau-lib.git) (not published to crates.io; see below for usage)

### Using the tagesschau_lib crate

This project uses a custom crate for fetching Tagesschau news.  
Add it to your `Cargo.toml` as a git dependency:

```toml
tagesschau_lib = { git = "https://code.mymiggi.de/Miggi/tagesschau-lib.git" }
```

### Running the Bot

1. **Clone this repository** and navigate to the project directory.

2. **Run the bot** with your Matrix credentials and server info:

   ```sh
   cargo run -- <homeserver_url> <username> <password>
   ```

   Example:

   ```sh
   cargo run -- https://matrix.org my-bot my-bot-password
   ```

3. **Invite the bot to a room**  
   Use a Matrix client to invite your bot to a room. The bot will auto-join.

4. **Try the commands!**
   - Post `!news` to receive the latest top 3 news headlines.
   - Post `!party` for some celebration.
   - Every day at 6:00 AM, the bot will send the latest news automatically.

### Example Output

```
🌐 Top 3 News Today | 30.05.2025

1. Headline 1
First sentence of headline 1
Tags: Politik, Wirtschaft
Source: https://www.tagesschau.de/news1

2. Headline 2
First sentence of headline 2
Tags: Ausland
Source: https://www.tagesschau.de/news2

3. Headline 3
First sentence of headline 3
Tags: Inland
Source: https://www.tagesschau.de/news3
```

## Configuration

- The Matrix room that receives daily news and the news update time (default: 6:00 AM) are currently hardcoded in the source code.
- **Planned:** Making both the room ID and the time for news updates configurable soon!

## How It Works

- Connects to your Matrix homeserver and logs in as the bot user.
- Listens for invites and joins rooms automatically.
- Listens for new messages and reacts to `!news` and `!party` commands.
- Every day at 6:00 AM, sends the latest Tagesschau news to the configured Matrix room.
- Fetches news using the `tagesschau_lib` crate.

## Extending

- **Change the daily news time:**  
  Edit the time in `start_news_thread`.
- **Send news to multiple rooms:**  
  Adapt the code to iterate over joined rooms.
- **Customize news format:**  
  Edit `print_news` and `build_news_msg` functions.

## Development

- Logging is available (uncomment the `tracing_subscriber::fmt::init()` line in `main()` to enable).
- Tests for time calculations are included in the `tests` module.

## Dependencies

- [matrix-sdk](https://crates.io/crates/matrix-sdk)
- [tokio](https://crates.io/crates/tokio)
- [chrono](https://crates.io/crates/chrono)
- [anyhow](https://crates.io/crates/anyhow)
- [tagesschau_lib](https://code.mymiggi.de/Miggi/tagesschau-lib.git)

## License

MIT

---

*Built with ❤️ in Rust for the German Matrix community!*