use crate::rss::{Rss, RssError};
use std::result::Result;
use worker::{Date, Error, Fetch, Method, Request};

pub struct SubscribedRSS {
    pub rss_url: String,
    pub tags: Vec<String>,
}

pub enum SubscribedError {
    RssError(RssError),
    WorkerError(Error),
}

impl SubscribedRSS {
    fn new(rss_url: impl Into<String>, tags: Vec<&str>) -> Self {
        Self {
            rss_url: rss_url.into(),
            tags: tags.iter().map(|&tag| tag.into()).collect(),
        }
    }

    pub async fn into_rss(self) -> Result<Rss, SubscribedError> {
        let request = match Request::new(&self.rss_url, Method::Get) {
            Ok(request) => request,
            Err(err) => {
                // TODO(#1) Inherite error information to log more detailed error
                return Err(SubscribedError::WorkerError(err));
            }
        };
        let mut response = match Fetch::Request(request).send().await {
            Ok(response) => response,
            Err(err) => {
                // TODO(#1) Inherite error information to log more detailed error
                return Err(SubscribedError::WorkerError(err));
            }
        };
        let rss_text = match response.text().await {
            Ok(rss_text) => rss_text,
            Err(err) => {
                // TODO(#1) Inherite error information to log more detailed error
                return Err(SubscribedError::WorkerError(err));
            }
        };

        match Rss::from_xml(&rss_text) {
            Ok(rss) => Ok(rss),
            Err(err) => Err(SubscribedError::RssError(err)),
        }
    }
}

pub fn get_subscribe_list() -> Vec<SubscribedRSS> {
    vec![SubscribedRSS::new("https://web.dev/feed.xml", vec!["web"])]
}
