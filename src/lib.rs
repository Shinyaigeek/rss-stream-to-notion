use serde_json::json;
use worker::*;

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

            for rss in list {
                let storedData = rss.into_store_schema();
            }

            Response::ok("hey")
        })
        .run(req, env)
        .await
}
