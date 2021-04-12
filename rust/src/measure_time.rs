use std::time::{Duration, Instant};
use chrono::{DateTime, Duration, Utc};

pub fn date_difference(date_time: DateTime<Utc>, yr:u8, mth:u8, dys:u16) -> Option<DateTime<Utc>> {
    date_time.checked_sub_signed(Duration::days(1))
}

pub fn date_summation(date_time: DateTime<Utc>, yr:u8, mth:u8, dys:u16) -> Option<DateTime<Utc>> {
    date_time.checked_add_signed(Duration::days(1))
}
pub fn measure_time(){
    let start = Instant::now();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

