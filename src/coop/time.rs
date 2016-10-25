extern crate time;

use time::Tm;

pub fn midnight() -> i64 {
    let mut now: Tm = time::now_utc();

    now.tm_hour = 0;
    now.tm_min = 0;
    now.tm_sec = 0;

    return now.to_timespec().sec;
}
