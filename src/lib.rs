use serde_json::json;
use worker::*;

mod latest_pushed_date_memory;
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

            for xml in list {
                let latest_pushed_date =
                    latest_pushed_date_memory::get_latest_pushed_date(&kv, &xml.rss_url.clone())
                        .await;
                let rss = match xml.into_rss().await {
                    Ok(rss) => rss,
                    Err(err) => return Response::error("internal server error", 500),
                };
                let latest_pushed_date_millis = match latest_pushed_date {
                    Some(latest_pushed_date) => latest_pushed_date.as_millis(),
                    None => 0,
                };
                let items = rss.items.iter().filter(|item| {
                    let item_published_date = match &item.published_date {
                        Some(item_published_date) => item_published_date.as_millis(),
                        None => 0,
                    };
                    item_published_date > latest_pushed_date_millis
                });
            }

            Response::ok("hey")
        })
        .run(req, env)
        .await
}
