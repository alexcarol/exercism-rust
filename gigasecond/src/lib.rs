extern crate chrono;
use chrono::*;

pub fn after(d: DateTime<UTC>) -> DateTime<UTC> {
    let duration = UTC.ymd(2043, 1, 1).and_hms(1,46,40) - UTC.ymd(2011, 4, 25).and_hms(0,0,0);

    d.checked_add(duration).unwrap()
}