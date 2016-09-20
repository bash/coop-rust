extern crate time;

use time::Tm;

// There's a  bug in the API that stores timestamps with a 2 hour offset
const OFFSET: i64 = 60 * 60 * 2;

pub fn midnight() -> i64 {
    let mut now: Tm = time::now_utc();

    now.tm_hour = 0;
    now.tm_min = 0;
    now.tm_sec = 0;

    return now.to_timespec().sec - OFFSET;
}