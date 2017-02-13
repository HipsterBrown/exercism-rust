extern crate chrono;
use chrono::prelude::*;
use chrono::Duration;

pub fn after(start_date: DateTime<UTC>) -> DateTime<UTC> {
    let base: i64 = 10;
    start_date + Duration::seconds(base.pow(9))
}
