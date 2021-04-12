use chrono::{DateTime, Duration, Utc};
fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_sub_signed(Duration::days(1))
}
pub fn date_difference(
    date_time: DateTime<Utc>,
    yr: i64,
    mth: i64,
    dys: i64,
) -> Option<DateTime<Utc>> {
    let obj: i64 = yr * 365 + mth * 30 + dys;
    date_time.checked_sub_signed(Duration::days(obj))
}

pub fn date_summation(
    date_time: DateTime<Utc>,
    yr: i64,
    mth: i64,
    dys: i64,
) -> Option<DateTime<Utc>> {
    let obj: i64 = yr * 365 + mth * 30 + dys;
    date_time.checked_add_signed(Duration::days(obj))
}
pub fn date_challenge() {
    let now = Utc::now();
    println!("{}", now);

    println!("{:?}", date_summation(now, 0, 1, 0));
    let almost_three_weeks_from_now = now
        .checked_add_signed(Duration::weeks(2))
        .and_then(|in_2weeks| in_2weeks.checked_add_signed(Duration::weeks(1)))
        .and_then(day_earlier);

    match almost_three_weeks_from_now {
        Some(x) => println!("{}", x),
        None => eprintln!("Almost three weeks from now overflows!"),
    }

    match now.checked_add_signed(Duration::max_value()) {
        Some(x) => println!("{}", x),
        None => eprintln!("We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."),
    }
}
