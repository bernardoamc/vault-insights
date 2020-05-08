use chrono::{DateTime, FixedOffset};
use chrono::offset::Utc;

pub struct Project {
    pub name: String,
    pub updated_at: Option<DateTime<FixedOffset>>,
    pub comment_url: Option<String>,
}

impl Project {
    pub fn parse(body: Option<&mut serde_json::value::Value>, vault_url: &str) -> Self {
        if body.is_none() {
            return Self { name: "Unknown...".to_owned(), updated_at: None, comment_url: None};
        }

        let body = body.unwrap();
        let name = &body["data"]["attributes"]["title"];

        let last_comment = match body["data"]["attributes"]["roadmap-comments"].as_array() {
            Some(comments) => {
                match comments.last() {
                    Some(comment) => comment,
                    None => {
                        return Self { name: name.to_string(), updated_at: None, comment_url: None };
                    }
                }
            },
            None =>  return Self { name: name.to_string(), updated_at: None, comment_url: None },
        };

        let id = &body["data"]["id"].to_string().replace("\"", "");
        let updated_at_str = last_comment["updated_at"].as_str().unwrap();
        let updated_at = DateTime::parse_from_str(&updated_at_str, "%Y-%m-%d %H:%M:%S %z").ok();
        let comment_url = format!("{}/projects/{}#status-update-{}", vault_url, id, last_comment["id"]);
        Self { name: name.to_string(), comment_url: Some(comment_url), updated_at }
    }

    pub fn is_updated(&self, since_days_ago: i64) -> bool {
        match self.updated_at {
            Some(updated_at) => {
                let days_since_update = (Utc::now().signed_duration_since(updated_at)).num_days();
                days_since_update <= since_days_ago
            },
            None => false
        }
    }
}
