use crate::addition::addition;
use crate::convert_age_to_days::convert_age_to_days;
mod addition;
mod convert_age_to_days;
fn main() {
    let mut _array = vec![addition(2, 3), convert_age_to_days(3)];
    println!("Hello, world!");
}
