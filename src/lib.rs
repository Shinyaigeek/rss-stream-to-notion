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
                let rss = xml.into_rss();
            }

            Response::ok("hey")
        })
        .run(req, env)
        .await
}
