use crate::rss::Rss;
use std::result::Result;
use worker::{Date, Fetch, Method, Request};

pub struct SubscribedRSS {
    pub rss_url: String,
    pub tags: Vec<String>,
}

impl SubscribedRSS {
    fn new(rss_url: impl Into<String>, tags: Vec<&str>) -> Self {
        Self {
            rss_url: rss_url.into(),
            tags: tags.iter().map(|&tag| tag.into()).collect(),
        }
    }

    pub async fn into_rss(self) -> Result<(), worker::Error> {
        let request = match Request::new(&self.rss_url, Method::Get) {
            Ok(request) => request,
            Err(err) => {
                // TODO(#1) Inherite error information to log more detailed error
                return Err(err);
            }
        };
        let mut response = match Fetch::Request(request).send().await {
            Ok(response) => response,
            Err(err) => {
                // TODO(#1) Inherite error information to log more detailed error
                return Err(err);
            }
        };
        let rss_text = match response.text().await {
            Ok(rss_text) => rss_text,
            Err(err) => {
                // TODO(#1) Inherite error information to log more detailed error
                return Err(err);
            }
        };

        return Ok(());
    }
}

pub fn get_subscribe_list() -> Vec<SubscribedRSS> {
    vec![SubscribedRSS::new("https://web.dev/feed.xml", vec!["web"])]
}
