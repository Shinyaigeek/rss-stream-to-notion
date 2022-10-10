use worker::Date;

struct Rss {
    pub id: String,
    pub blog_title: String,
    pub article_title: String,
    pub publishedDate: Date,
    pub article_url: String,
    pub rss_url: String,
    pub tags: Vec<String>,
    pub description: String,
}