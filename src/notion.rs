use crate::store::StoreSchema;
use serde::Serialize;
use wasm_bindgen::JsValue;
use worker::{Date, Error, Fetch, Headers, Method, Request, RequestInit};

const api_url_create_page: &str = "https://api.notion.com/v1/pages";
const api_version: &str = "2022-02-22";

#[derive(Debug)]
pub enum NotionCommandError {
    WorkerError(Error),
    SerializeError(serde_json::Error),
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
        let mut request_init = RequestInit::new();

        request_init.with_method(Method::Post);

        let mut headers = Headers::default();

        headers.append("Content-Type", "application/json").unwrap();
        headers.append("Authorization", &format!("Bearer {}", self.notion_api_key)).unwrap();
        headers.append("Notion-Version", api_version).unwrap();

        request_init.with_headers(headers);

        let notion_query = NotionQuery::from_store_schema(column, self.database_id.clone());

        let mut notion_query = match serde_json::to_string(&notion_query) {
            Ok(query) => query,
            // TODO(#1) Inherite error information to log more detailed error
            Err(err) => return Err(NotionCommandError::SerializeError(err)),
        };
        notion_query.remove_matches("__WILL_BE_REPLACED__");

        request_init.with_body(Some(JsValue::from_str(&notion_query)));

        let request = match Request::new_with_init(&api_url_create_page, &request_init) {
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

        let text = match response.text().await {
            Ok(text) => text,
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
    __WILL_BE_REPLACED__type: String,
    title: Vec<NotionTextQuery>,
}

#[derive(Serialize)]
struct NotionRichTextQuery {
    rich_text: Vec<NotionTextQuery>,
    __WILL_BE_REPLACED__type: String,
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
    blog_name: NotionRichTextQuery,
    article_title: NotionTitleQuery,
    tags: NotionMultiSelectQuery,
    guid: NotionRichTextQuery,
    description: NotionRichTextQuery,
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<NotionUrlQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    published_date: Option<NotionDateQuery>,
}

#[derive(Serialize)]
struct NotionQuery {
    parent: NotionParentQuery,
    properties: NotionPropertiesQuery,
}

impl NotionQuery {
    pub fn from_store_schema(store_schema: StoreSchema, database_id: String) -> Self {
        let multi_select: Vec<NotionSelectQuery> = store_schema
            .tags
            .iter()
            .map(|tag| NotionSelectQuery {
                name: tag.to_string(),
            })
            .collect();
        Self {
            parent: NotionParentQuery {
                __WILL_BE_REPLACED__type: "database_id".to_string(),
                database_id,
            },
            properties: NotionPropertiesQuery {
                article_title: NotionTitleQuery {
                    title: vec![NotionTextQuery {
                        __WILL_BE_REPLACED__type: "text".to_string(),
                        text: NotionContentQuery {
                            content: store_schema.article_title,
                        },
                    }],
                    __WILL_BE_REPLACED__type: "title".to_string(),
                },
                blog_name: NotionRichTextQuery {
                    rich_text: vec![NotionTextQuery {
                        __WILL_BE_REPLACED__type: "text".to_string(),
                        text: NotionContentQuery {
                            content: store_schema.blog_title,
                        },
                    }],
                    __WILL_BE_REPLACED__type: "rich_text".to_string(),
                },
                tags: NotionMultiSelectQuery { multi_select },
                guid: NotionRichTextQuery {
                    rich_text: vec![NotionTextQuery {
                        __WILL_BE_REPLACED__type: "text".to_string(),
                        text: NotionContentQuery {
                            content: store_schema.guid,
                        },
                    }],
                    __WILL_BE_REPLACED__type: "rich_text".to_string(),
                },
                description: NotionRichTextQuery {
                    rich_text: vec![NotionTextQuery {
                        __WILL_BE_REPLACED__type: "text".to_string(),
                        text: NotionContentQuery {
                            content: store_schema.description.clone(),
                        },
                    }],
                    __WILL_BE_REPLACED__type: "rich_text".to_string(),
                },
                link: match store_schema.link {
                    Some(link) => Some(NotionUrlQuery { url: link }),
                    None => None,
                },
                published_date: match store_schema.published_date {
                    Some(published_date) => Some(NotionDateQuery {
                        date: NotionStartDateQuery {
                            start: published_date.to_string(),
                        },
                    }),
                    None => None,
                },
            },
        }
    }
}
