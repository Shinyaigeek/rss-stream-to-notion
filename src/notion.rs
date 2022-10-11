use serde::{Serialize};
use crate::store::StoreSchema;
use worker::{Date, Error, Fetch, Headers, Method, Request, RequestInit};

const api_url_create_page: &str = "https://api.notion.com/v1/pages";
const api_version: &str = "2022-06-28";

pub enum NotionCommandError {
    WorkerError(Error),
    SerializeError(serde_json::Error)
}

pub struct NotionCommand {
    pub database_id: String,
    pub notify_user_id: String,
    pub notion_api_key: String,
}

impl NotionCommand {
    pub fn build(database_id: String, notify_user_id: String, notion_api_key: String) -> Self {
        Self {
            database_id,
            notify_user_id,
            notion_api_key,
        }
    }

    pub async fn insert_column(&self, column: StoreSchema) -> Result<(), NotionCommandError> {
        let mut request_init = RequestInit::default();

        request_init.with_method(Method::Post);

        let mut headers = Headers::default();

        headers.append("Content-Type", "application/json");
        headers.append("Authorization", &format!("Bearer {}", self.notion_api_key));
        headers.append("Notion-Version", api_version);

        request_init.with_headers(headers);

        let notion_query = NotionQuery::from_store_schema(column, self.database_id);

        let mut notion_query = match serde_json::to_string(&notion_query) {
            Ok(query) => query,
            // TODO(#1) Inherite error information to log more detailed error
            Err(err) => return Err(NotionCommandError::SerializeError(err))
        };
        notion_query.remove_matches("__WILL_BE_REPLACED__");

        request_init.with_body(notion_query.into());

        let mut request = match Request::new_with_init(&api_url_create_page, &request_init) {
            Ok(request) => request,
            Err(err) => {
                // TODO(#1) Inherite error information to log more detailed error
                return Err(NotionCommandError::WorkerError(err));
            }
        };

        let mut response = match Fetch::Request(request).send().await {
            Ok(response) => response,
            Err(err) => {
                // TODO(#1) Inherite error information to log more detailed error
                return Err(NotionCommandError::WorkerError(err));
            }
        };

        let rss_text = match response.text().await {
            Ok(rss_text) => rss_text,
            Err(err) => {
                // TODO(#1) Inherite error information to log more detailed error
                return Err(NotionCommandError::WorkerError(err));
            }
        };

        Ok(())
    }
}

#[derive(Serialize)]
struct NotionParentQuery {
    __WILL_BE_REPLACED__type: String,
    database_id: String,
}

#[derive(Serialize)]
struct NotionContentQuery {
    content: String,
}

#[derive(Serialize)]
struct NotionTextQuery {
    __WILL_BE_REPLACED__type: String,
    text: NotionContentQuery,
}

#[derive(Serialize)]
struct NotionTitleQuery {
    text: Vec<NotionTextQuery>,
}

#[derive(Serialize)]
struct NotionRichTextQuery {
    rich_text: Vec<NotionTextQuery>,
}

#[derive(Serialize)]
struct NotionStartDateQuery {
    start: String,
}

#[derive(Serialize)]
struct NotionDateQuery {
    date: NotionStartDateQuery,
}

#[derive(Serialize)]
struct NotionUrlQuery {
    url: String,
}

#[derive(Serialize)]
struct NotionSelectQuery {
    name: String,
}

#[derive(Serialize)]
struct NotionMultiSelectQuery {
    multi_select: Vec<NotionSelectQuery>,
}

#[derive(Serialize)]
struct NotionChildQuery {
    object: String,
    __WILL_BE_REPLACED__type: String,
    paragraph: NotionRichTextQuery,
}

#[derive(Serialize)]
struct NotionPropertiesQuery {
    blog_title: NotionRichTextQuery,
    article_title: NotionTitleQuery,
    tags: NotionMultiSelectQuery,
    guid: NotionRichTextQuery,
    description: NotionRichTextQuery,
    link: NotionUrlQuery,
    published_date: NotionDateQuery,
}

#[derive(Serialize)]
struct NotionQuery {
    parent: NotionParentQuery,
    properties: NotionPropertiesQuery,
    children: Vec<NotionChildQuery>,
}

impl NotionQuery {
    pub fn from_store_schema(store_schema: StoreSchema, database_id: String) -> Self {
        let multi_select: Vec<NotionSelectQuery> = store_schema
            .tags
            .iter()
            .map(|tag| NotionSelectQuery { name: tag.to_string() })
            .collect();
        Self {
            parent: NotionParentQuery {
                __WILL_BE_REPLACED__type: "database_id".to_string(),
                database_id,
            },
            properties: NotionPropertiesQuery {
                blog_title: NotionRichTextQuery {
                    rich_text: vec![NotionTextQuery {
                        __WILL_BE_REPLACED__type: "text".to_string(),
                        text: NotionContentQuery {
                            content: store_schema.blog_title,
                        },
                    }],
                },
                article_title: NotionTitleQuery {
                    text: vec![NotionTextQuery {
                        __WILL_BE_REPLACED__type: "text".to_string(),
                        text: NotionContentQuery {
                            content: store_schema.article_title,
                        },
                    }],
                },
                tags: NotionMultiSelectQuery { multi_select },
                guid: NotionRichTextQuery {
                    rich_text: vec![NotionTextQuery {
                        __WILL_BE_REPLACED__type: "text".to_string(),
                        text: NotionContentQuery {
                            content: store_schema.guid,
                        },
                    }],
                },
                description: NotionRichTextQuery {
                    rich_text: vec![NotionTextQuery {
                        __WILL_BE_REPLACED__type: "text".to_string(),
                        text: NotionContentQuery {
                            content: store_schema.description,
                        },
                    }],
                },
                link: NotionUrlQuery {
                    url: store_schema.link,
                },
                published_date: NotionDateQuery {
                    date: NotionStartDateQuery {
                        start: store_schema.published_date,
                    },
                },
            },
            children: vec![NotionChildQuery {
                object: "block".to_string(),
                __WILL_BE_REPLACED__type: "paragraph".to_string(),
                paragraph: NotionRichTextQuery {
                    rich_text: vec![NotionTextQuery {
                        __WILL_BE_REPLACED__type: "text".to_string(),
                        text: NotionContentQuery {
                            content: store_schema.description,
                        },
                    }],
                },
            }],
        }
    }
}
