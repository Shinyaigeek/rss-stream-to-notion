use insta;
use roxmltree::{Document, Error};
use std::result::Result;
use worker::{Date, DateInit};

#[derive(Debug)]
pub struct RssItem {
    pub id: String,
    pub blog_title: String,
    pub article_title: String,
    pub published_date: Option<Date>,
    pub article_url: Option<String>,
    pub categories: Vec<String>,
    pub description: String,
}

impl RssItem {
    pub fn new(
        id: impl Into<String>,
        blog_title: impl Into<String>,
        article_title: impl Into<String>,
        published_date: Option<Date>,
        article_url: Option<String>,
        categories: Vec<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            blog_title: blog_title.into(),
            article_title: article_title.into(),
            published_date,
            article_url,
            categories,
            description: description.into(),
        }
    }
}

#[derive(Debug)]
enum RssError {
    Xml(Error),
    Markup(String),
}

#[derive(Debug)]
pub struct Rss {
    pub items: Vec<RssItem>,
}

impl Rss {
    fn from_xml(xml: &str) -> Result<Self, RssError> {
        let document = match Document::parse(&xml) {
            Ok(document) => document,
            Err(err) => return Err(RssError::Xml(err)),
        };

        if document.root_element().has_tag_name("rss") {
            Self::from_rss_element_root(document)
        } else {
            Self::from_feed_element_root(document)
        }
    }

    fn from_rss_element_root(document: Document) -> Result<Self, RssError> {
        let root = document.root_element();
        let channel = match root.children().find(|child| child.has_tag_name("channel")) {
            Some(channel) => channel,
            None => {
                return Err(RssError::Markup(
                    "rss root element should have channel element in its children".into(),
                ))
            }
        };

        let blog_title_element = match channel.children().find(|child| child.has_tag_name("title"))
        {
            Some(blog_title_element) => blog_title_element,
            None => {
                return Err(RssError::Markup(
                    "rss channel element should have title element in its children".into(),
                ))
            }
        };

        let blog_title = match blog_title_element.text() {
            Some(blog_title) => blog_title,
            None => {
                return Err(RssError::Markup(
                    "title element should have text content".into(),
                ))
            }
        };

        let root_categories_elements = channel
            .children()
            .filter(|child| child.has_tag_name("category"));

        let root_categories = root_categories_elements
            .map(
                |root_categories_element| match root_categories_element.text() {
                    Some(root_category) => root_category.to_string(),
                    None => "".to_string(),
                },
            )
            .filter(|root_category| root_category.len() > 0);

        let items = channel
            .children()
            .filter(|child| child.has_tag_name("item"));

        let items: Vec<RssItem> = items
            .map(|item| {
                let article_title_element =
                    match item.children().find(|child| child.has_tag_name("title")) {
                        Some(article_title_element) => article_title_element,
                        None => {
                            return Err(RssError::Markup(
                                "item element should have title element in its children".into(),
                            ))
                        }
                    };

                let article_title = match article_title_element.text() {
                    Some(article_title) => article_title,
                    None => {
                        return Err(RssError::Markup(
                            "title element should have text content".into(),
                        ))
                    }
                };

                let description_element = match item
                    .children()
                    .find(|child| child.has_tag_name("description"))
                {
                    Some(description_element) => description_element,
                    None => {
                        return Err(RssError::Markup(
                            "item element should have description element in its children".into(),
                        ))
                    }
                };

                let description = match description_element.text() {
                    Some(description) => description,
                    None => {
                        return Err(RssError::Markup(
                            "description element should have text content".into(),
                        ))
                    }
                };

                let article_url = match item.children().find(|child| child.has_tag_name("link")) {
                    Some(link_element) => match link_element.text() {
                        Some(link) => Some(link.to_string()),
                        None => None,
                    },
                    None => None,
                };

                let id = match item.children().find(|child| child.has_tag_name("guid")) {
                    Some(guid_element) => match guid_element.text() {
                        Some(guid) => guid.to_string(),
                        None => {
                            format!("{}__{}", blog_title.to_string(), article_title.to_string())
                        }
                    },
                    None => format!("{}__{}", blog_title.to_string(), article_title.to_string()),
                };

                let published_date =
                    match item.children().find(|child| child.has_tag_name("pubDate")) {
                        Some(published_date_element) => match published_date_element.text() {
                            Some(published_date) => {
                                Some(Date::new(DateInit::String(published_date.into())))
                            }
                            None => None,
                        },
                        None => None,
                    };

                let article_categories_elements = item
                    .children()
                    .filter(|child| child.has_tag_name("category"));

                let article_categories = article_categories_elements
                    .map(
                        |article_categories_element| match article_categories_element.text() {
                            Some(article_category) => article_category.to_string(),
                            None => "".to_string(),
                        },
                    )
                    .filter(|article_category| article_category.len() > 0);

                let mut categories = article_categories.collect::<Vec<String>>();

                categories.extend_from_slice(&root_categories.clone().collect::<Vec<String>>());

                Ok(RssItem::new(
                    id,
                    blog_title,
                    article_title,
                    published_date,
                    article_url,
                    categories,
                    description,
                ))
            })
            .filter(|item| item.is_ok())
            .map(|item| item.unwrap())
            .collect();

        Ok(Self { items })
    }

    fn from_feed_element_root(document: Document) -> Result<Self, RssError> {
        let feed = document.root_element();

        let blog_title_element = match feed.children().find(|child| child.has_tag_name("title")) {
            Some(blog_title_element) => blog_title_element,
            None => {
                return Err(RssError::Markup(
                    "rss feed element should have title element in its children".into(),
                ))
            }
        };

        let blog_title = match blog_title_element.text() {
            Some(blog_title) => blog_title,
            None => {
                return Err(RssError::Markup(
                    "title element should have text content".into(),
                ))
            }
        };

        let root_categories_elements = feed
            .children()
            .filter(|child| child.has_tag_name("category"));

        let root_categories = root_categories_elements
            .map(
                |root_categories_element| match root_categories_element.text() {
                    Some(root_category) => root_category.to_string(),
                    None => "".to_string(),
                },
            )
            .filter(|root_category| root_category.len() > 0);

        let items = feed
            .children()
            .filter(|child| child.has_tag_name("item") || child.has_tag_name("entry"));

        let items: Vec<RssItem> = items
            .map(|item| {
                let article_title_element =
                    match item.children().find(|child| child.has_tag_name("title")) {
                        Some(article_title_element) => article_title_element,
                        None => {
                            return Err(RssError::Markup(
                                "item element should have title element in its children".into(),
                            ))
                        }
                    };

                let article_title = match article_title_element.text() {
                    Some(article_title) => article_title,
                    None => {
                        return Err(RssError::Markup(
                            "title element should have text content".into(),
                        ))
                    }
                };

                let description_element = match item
                    .children()
                    .find(|child| child.has_tag_name("description"))
                {
                    Some(description_element) => description_element,
                    None => {
                        return Err(RssError::Markup(
                            "item element should have description element in its children".into(),
                        ))
                    }
                };

                let description = match description_element.text() {
                    Some(description) => description,
                    None => {
                        return Err(RssError::Markup(
                            "description element should have text content".into(),
                        ))
                    }
                };

                let article_url = match item.children().find(|child| child.has_tag_name("link")) {
                    Some(link_element) => match link_element.text() {
                        Some(link) => Some(link.to_string()),
                        None => None,
                    },
                    None => None,
                };

                let id = match item.children().find(|child| child.has_tag_name("guid")) {
                    Some(guid_element) => match guid_element.text() {
                        Some(guid) => guid.to_string(),
                        None => {
                            format!("{}__{}", blog_title.to_string(), article_title.to_string())
                        }
                    },
                    None => format!("{}__{}", blog_title.to_string(), article_title.to_string()),
                };

                let published_date =
                    match item.children().find(|child| child.has_tag_name("pubDate")) {
                        Some(published_date_element) => match published_date_element.text() {
                            Some(published_date) => {
                                Some(Date::new(DateInit::String(published_date.into())))
                            }
                            None => None,
                        },
                        None => None,
                    };

                let article_categories_elements = item
                    .children()
                    .filter(|child| child.has_tag_name("category"));

                let article_categories = article_categories_elements
                    .map(
                        |article_categories_element| match article_categories_element.text() {
                            Some(article_category) => article_category.to_string(),
                            None => "".to_string(),
                        },
                    )
                    .filter(|article_category| article_category.len() > 0);

                let mut categories = article_categories.collect::<Vec<String>>();

                categories.extend_from_slice(&root_categories.clone().collect::<Vec<String>>());

                Ok(RssItem::new(
                    id,
                    blog_title,
                    article_title,
                    published_date,
                    article_url,
                    categories,
                    description,
                ))
            })
            .filter(|item| item.is_ok())
            .map(|item| item.unwrap())
            .collect();

        Ok(Self { items })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_with_rss_root_element_xml() {
        let rss = Rss::from_xml("<rss xmlns:dc=\"http://purl.org/dc/elements/1.1/\" xmlns:content=\"http://purl.org/rss/1.0/modules/content/\" xmlns:atom=\"http://www.w3.org/2005/Atom\" xmlns:media=\"http://search.yahoo.com/mrss/\" version=\"2.0\"><channel><title>blog title</title><item><title>article title 1</title><description>article description 1</description><link>link</link><guid>guid</guid></item><item><title>article title 2</title><description>article description 2</description><link>link</link><guid>guid</guid></item></channel></rss>");
        insta::assert_debug_snapshot!(rss);
    }

    #[test]
    fn initialize_with_feed_root_element_xml() {
        let rss = Rss::from_xml("<feed xmlns:dc=\"http://purl.org/dc/elements/1.1/\" xmlns:content=\"http://purl.org/rss/1.0/modules/content/\" xmlns:atom=\"http://www.w3.org/2005/Atom\" xmlns:media=\"http://search.yahoo.com/mrss/\" version=\"2.0\"><title>blog title</title><item><title>article title 1</title><description>article description 1</description><link>link</link><guid>guid</guid></item><item><title>article title 2</title><description>article description 2</description><link>link</link><guid>guid</guid></item></feed>");
        insta::assert_debug_snapshot!(rss);
    }

    #[test]
    fn initialize_with_feed_root_element_xml_with_entry_tag() {
        let rss = Rss::from_xml("<feed xmlns:dc=\"http://purl.org/dc/elements/1.1/\" xmlns:content=\"http://purl.org/rss/1.0/modules/content/\" xmlns:atom=\"http://www.w3.org/2005/Atom\" xmlns:media=\"http://search.yahoo.com/mrss/\" version=\"2.0\"><title>blog title</title><entry><title>article title 1</title><description>article description 1</description><link>link</link><guid>guid</guid></entry><entry><title>article title 2</title><description>article description 2</description><link>link</link><guid>guid</guid></entry></feed>");
        insta::assert_debug_snapshot!(rss);
    }

    #[test]
    fn generate_guid_with_title_if_it_is_none() {
        let rss = Rss::from_xml("<rss xmlns:dc=\"http://purl.org/dc/elements/1.1/\" xmlns:content=\"http://purl.org/rss/1.0/modules/content/\" xmlns:atom=\"http://www.w3.org/2005/Atom\" xmlns:media=\"http://search.yahoo.com/mrss/\" version=\"2.0\"><channel><title>blog title</title><item><title>article title 1</title><description>article description 1</description><link>link</link></item></channel></rss>");
        assert_eq!(rss.unwrap().items[0].id, "blog title__article title 1")
    }

    #[test]
    fn merge_categories() {
        let rss = Rss::from_xml("<rss xmlns:dc=\"http://purl.org/dc/elements/1.1/\" xmlns:content=\"http://purl.org/rss/1.0/modules/content/\" xmlns:atom=\"http://www.w3.org/2005/Atom\" xmlns:media=\"http://search.yahoo.com/mrss/\" version=\"2.0\"><channel><title>blog title</title><category>root 1</category><category>root 2</category><item><title>article title 1</title><category>article 1</category><category>article 2</category><description>article description 1</description><link>link</link></item></channel></rss>");
        assert_eq!(
            rss.unwrap().items[0].categories,
            vec![
                "article 1".to_string(),
                "article 2".to_string(),
                "root 1".to_string(),
                "root 2".to_string(),
            ]
        )
    }
}
