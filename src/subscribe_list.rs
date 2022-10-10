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
}

pub fn get_subscribe_list() -> Vec<SubscribedRSS> {
    vec![
        SubscribedRSS::new("https://en.shinyaigeek.dev/rss.xml", vec!["mine", "web"]),
        SubscribedRSS::new("https://web.dev/feed.xml", vec!["web"]),
    ]
}
