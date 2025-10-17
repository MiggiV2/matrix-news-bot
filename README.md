# Tagesschau Matrix Bot

A simple Rust-powered Matrix bot that sends you the latest news from Germany's leading news portal [Tagesschau](https://www.tagesschau.de/).

## Features

- **Automated News Updates**: The bot posts the top 3 news stories from Tagesschau at your configured time and frequency to your chosen Matrix room.
- **On-Demand News**: Send the command `!news` in any room the bot is in, and it will respond with the latest headlines.
- **Auto-Join**: The bot will automatically join rooms it is invited to.

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

2. **Configure the bot** by creating a `config.toml` file (see Configuration section below).

3. **Run the bot**:

   ```sh
   cargo run
   ```

4. **Invite the bot to a room**  
   Use a Matrix client to invite your bot to a room. The bot will auto-join.

5. **Try the commands!**
   - Post `!news` to receive the latest top 3 news headlines.
   - The bot will send the latest news automatically according to your configured schedule.

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

The bot is configured via a `config.toml` file in the project root. See `config.toml.example` for a template.

### Configuration Options

```toml
# Matrix configuration
matrix_homerserver = "https://matrix.org"
matrix_username = "news"
matrix_password = "very_secret"
matrix_room_id = "!your_room_id:matrix.org"

# News configuration
news_time = "06:00"           # Time to send daily news (HH:MM format)
update_frequency = "24h"      # How often to send news (e.g., "24h", "12h", "30m")

# Optional
bot_name = "Tagesschau_Bot"   # Display name for the bot
```

- **news_time**: The time of day when the first news update is sent (24-hour format, HH:MM)
- **update_frequency**: How often to send news updates after the first one. Supports:
  - Hours: `"24h"`, `"12h"`, etc.
  - Minutes: `"30m"`, `"60m"`, etc.

## How It Works

- Connects to your Matrix homeserver and logs in as the bot user.
- Listens for invites and joins rooms automatically.
- Listens for new messages and reacts to the `!news` command.
- Sends the latest Tagesschau news to the configured Matrix room at the specified time and frequency.
- Fetches news using the `tagesschau_lib` crate.

## Extending

- **Change the daily news time:**  
  Edit `news_time` in `config.toml`.
- **Change the update frequency:**  
  Edit `update_frequency` in `config.toml`.
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
- [serde](https://crates.io/crates/serde)
- [toml](https://crates.io/crates/toml)
- [tagesschau_lib](https://code.mymiggi.de/Miggi/tagesschau-lib.git)

## License

MIT

---

*Built with ❤️ in Rust for the German Matrix community!*