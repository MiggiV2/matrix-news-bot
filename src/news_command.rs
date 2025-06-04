use chrono::Utc;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use tagesschau_lib::{News, Tagesschau};

pub async fn build_news_msg() -> RoomMessageEventContent {
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