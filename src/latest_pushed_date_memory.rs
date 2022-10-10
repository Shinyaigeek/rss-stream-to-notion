use durable::Storage;
use worker::{Date, DateInit};

pub fn get_latest_pushed_date(key: String) -> Option<Date> {
    match Storage::get(key) {
        Ok(latest_pushed_data) => Date::from(DateInit::String(latest_pushed_data.to_string())),
        Err => None,
    }
}

pub fn put_latest_pushed_date(key: String, latest_pushed_date: Date) -> Result<()> {
    Storage.put(key, latest_pushed_date.to_string())
}
