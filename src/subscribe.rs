use crate::store::StoreSchema;
use roxmltree::Document;
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

    pub async fn into_store_schema(self) -> StoreSchema {
        let request = Request::new(&self.rss_url, Method::Get).unwrap();
        let mut response = Fetch::Request(request).send().await.unwrap();
        let text = response.text().await.unwrap();
        let doc = Document::parse(&text).unwrap();
        let title = doc
            .descendants()
            .find(|n| n.has_tag_name("title"))
            .unwrap()
            .text()
            .unwrap();
        let description = doc
            .descendants()
            .find(|n| n.has_tag_name("description"))
            .unwrap()
            .text()
            .unwrap();

        StoreSchema::new(
            title,
            &self.rss_url,
            self.tags,
            description,
            &self.rss_url,
            Date::now(),
        )
    }
}

pub fn get_subscribe_list() -> Vec<SubscribedRSS> {
    vec![SubscribedRSS::new("https://web.dev/feed.xml", vec!["web"])]
}
