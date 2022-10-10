use worker::Date;

struct RssItem {
    pub id: String,
    pub blog_title: String,
    pub article_title: String,
    pub publishedDate: Date,
    pub article_url: String,
    pub rss_url: String,
    pub tags: Vec<String>,
    pub description: String,
}

impl RssItem {
    pub fn new(
        id: impl Into<String>,
        blog_title: impl Into<String>,
        article_title: impl Into<String>,
        publishedDate: Date,
        article_url: impl Into<String>,
        rss_url: impl Into<String>,
        tags: Vec<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            blog_title: blog_title.into(),
            article_title: article_title.into(),
            publishedDate,
            article_url: article_url.into(),
            rss_url: rss_url.into(),
            tags,
            description: description.into(),
        }
    }
}

pub struct Rss {
    items: Vec<RssItem>,
}
