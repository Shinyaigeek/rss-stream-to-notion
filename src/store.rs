use rand::Rng;
use worker::{Date, DateInit};

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
abcdefghijklmnopqrstuvwxyz\
0123456789";
const GUID_LEN: usize = 30;

pub struct StoreSchema {
    pub blog_title: String,
    pub article_title: String,
    pub rss_url: String,
    pub tags: Vec<String>,
    pub description: String,
    pub read: bool,
    pub guid: String,
    pub link: Option<String>,
    pub published_date: Option<Date>,
}

impl StoreSchema {
    pub fn new(
        guid: impl Into<String>,
        blog_title: impl Into<String>,
        article_title: impl Into<String>,
        rss_url: impl Into<String>,
        tags: Vec<String>,
        description: impl Into<String>,
        link: &Option<String>,
        published_date: &Option<Date>,
    ) -> Self {
        let mut rng = rand::thread_rng();

        let guid: String = (0..GUID_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        Self {
            blog_title: blog_title.into(),
            article_title: article_title.into(),
            rss_url: rss_url.into(),
            tags,
            description: description.into(),
            read: false,
            guid,
            link: link.clone(),
            published_date: match published_date {
                Some(published_date) => {
                    Some(Date::new(DateInit::Millis(published_date.as_millis())))
                }
                None => None,
            },
        }
    }
}
