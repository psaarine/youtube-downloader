use std::vec::Vec;
use serde::Deserialize;
use websocket::OwnedMessage;

#[derive(Deserialize)]
pub struct ApplicationMessage {

    pub list_name: String,
    pub uris: Vec<String>
}

impl ApplicationMessage {

    pub fn from_string(target: String) -> Option<Self> {

        let msg = serde_json::from_str(&target);
        return msg.ok();
    }

    pub fn from_owned_message(msg: OwnedMessage) -> Option<Self> {

        if let OwnedMessage::Text(content) = msg {

            return ApplicationMessage::from_string(content);
        } else {

            return None;
        }
    }
}

#[cfg(test)]
mod application_message_tests {

    use super::*;

    fn get_data_as_string() -> String {

        let data = r#"
        {
            "list_name": "basic_downloads",
            "uris": [

                "test.com",
                "lol.com"
            ]
        }"#;

        return data.to_owned();
    }
    #[test]
    fn test_simple_deserialize() {


        let data = get_data_as_string();
        let msg: ApplicationMessage = serde_json::from_str(&data).unwrap();

        assert_eq!(msg.list_name, "basic_downloads".to_owned());
        assert_eq!(msg.uris.len(), 2);
        assert_eq!(msg.uris[0], "test.com".to_owned());
        assert_eq!(msg.uris[1], "lol.com".to_owned());
    }
}
