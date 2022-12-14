#![feature(string_remove_matches)]

use serde_json;
use worker::*;

mod latest_pushed_date_memory;
mod notion;
mod rss;
mod store;
mod subscribe;
mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);
    utils::set_panic_hook();
    let router = Router::new();
    router
        .get_async("/", |mut req, ctx| async move {
            let list = subscribe::get_subscribe_list();

            let kv = ctx.kv("LATEST_PUSHED_DATES")?;

            let database_id = ctx.secret("database_id").unwrap().to_string();
            let notify_user_id = ctx.secret("notify_user_id").unwrap().to_string();
            let notion_api_key = ctx.secret("notion_api_key").unwrap().to_string();

            let notion_query = notion::NotionCommand::build(database_id, notify_user_id, notion_api_key);

            for xml in list {
                let rss_url = xml.rss_url.clone();
                let tags = xml.tags.clone();

                let latest_pushed_date =
                    latest_pushed_date_memory::get_latest_pushed_date(&kv, &rss_url).await;
                let rss = match xml.into_rss().await {
                    Ok(rss) => rss,
                    Err(err) => return Response::error("internal server error", 500),
                };
                let latest_pushed_date_millis = match latest_pushed_date {
                    Some(latest_pushed_date) => latest_pushed_date.as_millis(),
                    None => 0,
                };
                let latest_pushed_date = rss.exclude_latest_published_date();
                let items = rss.items.iter().filter(|item| {
                    let item_published_date = match &item.published_date {
                        Some(item_published_date) => item_published_date.as_millis(),
                        None => 0,
                    };
                    true
                });

                let store_columns = items.map(|item| {
                    store::StoreSchema::new(
                        &item.id,
                        &item.blog_title,
                        &item.article_title,
                        rss_url.clone(),
                        tags.clone(),
                        &item.description,
                        &item.article_url,
                        &item.published_date,
                    )
                });

                for store_column in store_columns {
                    let res = notion_query.insert_column(store_column).await;
                    match res {
                        Ok(_) => {},
                        Err(err) => worker::console_log!("err: {:?}", err)
                    };
                }

                latest_pushed_date_memory::put_latest_pushed_date(
                    &kv,
                    &rss_url,
                    latest_pushed_date,
                )
                .await;
            }

            Response::ok("ok")
        })
        .run(req, env)
        .await
}
