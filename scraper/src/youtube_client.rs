pub struct YoutubeClient {

    http_client: reqwest::Client
}

impl YoutubeClient {

    pub fn new() -> reqwest::Result<Self> {

        let client = reqwest::Client::builder()
            .default_headers(get_default_headers())
            .build();

        if let Err(e) = client {

            return Err(e);
        }

        let client = Self { http_client: client.unwrap() };

        return Ok(client);
    }

    pub async fn download_html<T: AsRef<str>>(&self, target_url: T) ->  reqwest::Result<()>{

        // Get the video id from url
        


        // Get watch url from video id

        // Get filesize from watch url
        unimplemented!();
    }

}

fn get_default_headers() -> reqwest::header::HeaderMap {

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", reqwest::header::HeaderValue::from_static("Mozilla/5.0"));
    headers.insert("accept-language", reqwest::header::HeaderValue::from_static("en-US,en"));

    return headers;
}
