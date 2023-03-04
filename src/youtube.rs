use color_eyre::{eyre::eyre, Report};
use reqwest::Client;
use tap::Pipe;
use url::Url;

const YT_CHANNEL_ID: &str = "UCs4fQRyl1TJvoeOdekW6lYA";

#[tracing::instrument]
pub(crate) async fn fetch_video_id(client: &Client) -> Result<String, Report> {
    let mut api_uri = Url::parse("https://www.youtube.com/feeds/videos.xml")?;
    {
        let mut query = api_uri.query_pairs_mut();
        query.append_pair("channel_id", YT_CHANNEL_ID);
    }

    // let client = Client::new();
    let res = client
        .get(api_uri)
        // add header for http header
        .header("user-agent", "cool/yyxx")
        .send()
        .await? // will cover connection error
        .error_for_status()? // will cover http error
        .bytes()
        .await? // errors while streaming the response body?
        // parse the feed
        .pipe(|bytes| feed_rs::parser::parse(&bytes[..]))?
        .pipe_ref(|feed| feed.entries.get(0))
        .ok_or(eyre!("No video found in channel"))?
        .pipe_ref(|entry| entry.id.strip_prefix("yt:video:"))
        .ok_or(eyre!("first video feed item wasn't a video"))?
        .to_string();
    Ok(res)
}
