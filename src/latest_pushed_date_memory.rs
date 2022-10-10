use worker::{
    kv::{KvError, KvStore},
    Date, DateInit,
};

pub async fn get_latest_pushed_date(kv: &KvStore, key: &str) -> Option<Date> {
    match kv.get(key).text().await {
        Ok(latest_pushed_data) => match latest_pushed_data {
            Some(latest_pushed_data) => {
                Some(Date::from(DateInit::String(latest_pushed_data.to_string())))
            }
            None => None,
        },
        Err(_) => None,
    }
}

pub async fn put_latest_pushed_date(
    kv: &KvStore,
    key: &str,
    latest_pushed_date: Date,
) -> Result<(), KvError> {
    let put_command = match kv.put(key, latest_pushed_date.to_string()) {
        Ok(cmd) => cmd,
        Err(err) => return Err(err),
    };

    put_command.execute().await
}
