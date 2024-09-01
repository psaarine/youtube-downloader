const DEFAULT_VIDEO_SIZE: u32 = 9437184;
const VIDEO_ID_REGEX: &str = r#"(?:v=|\/)([0-9A-Za-z_-]{11}).*"#;

pub struct VideoMetadata {

    video_len: String
}

async fn get_video_metadata<T: reqwest::IntoUrl>(client: &reqwest::Client, url: T) -> Option<VideoMetadata> {

    /*
    let request = client.head(url).build();

    if let Err(_e) = request { return None };

    let request = request.unwrap();
    client.execute(request).await;

    return None;
    */
    unimplemented!();
}

pub fn create_watch_url<T: std::fmt::Display>(video_id: T) -> String {

    return std::format!("https://youtube.com/watch?v={}", video_id);
}

pub fn create_embed_url<T: std::fmt::Display>(video_id: T) -> String {

    return std::format!("https://youtube.com/embed/{}", video_id);
}

pub struct VideoIdParser {

    parser_regex: regex::Regex
}

impl VideoIdParser {

    pub fn new() -> Self {
        let parser_regex = regex::Regex::new(VIDEO_ID_REGEX).unwrap();
        return Self { parser_regex: parser_regex }
    }

    pub fn try_parse_video_id(&self, video_url: &str) -> Option<String> {

        let resp = self.parser_regex.find(video_url);
        if let None = resp {

            return None;
        }

        return Some(resp.unwrap().as_str().to_owned());
    }
}

