use crate::addition::addition;
use crate::alphabet_soup::alphabet_soup;
use crate::convert_age_to_days::convert_age_to_days;
use crate::convert_days_to_age::convert_days_to_age;
use crate::convert_minutes_to_seconds::convert_minutes_to_seconds;
use crate::cubes::cubes;
// use crate::find_duplicates::find_duplicates;
use crate::first_char::first_char;
// use crate::fizz_buzz::fizz_buzz;
use crate::hello::hello;
use crate::next_integer::next_integer;
// use crate::read_slice::read_slice;
// use crate::reduce_array::reduce_array;
use crate::remove_first_char::remove_first_char;
use crate::remove_spaces::remove_spaces;
// use crate::reverse_words::reverse_words;
mod addition;
mod alphabet_soup;
mod convert_age_to_days;
mod convert_days_to_age;
mod convert_minutes_to_seconds;
mod cubes;
// mod find_duplicates;
mod first_char;
// mod fizz_buzz;
mod hello;
mod next_integer;
// mod read_slice;
// mod reduce_array;
mod remove_first_char;
mod remove_spaces;
// mod reverse_words;
#[derive(Debug)]
struct Person {
    name: String,
}
impl Person {
    fn new<S: Into<String>>(name: S) -> Person {
        Person { name: name.into() }
    }
}
fn main() {
    let string_array = vec![
        remove_spaces("hello"),
        remove_first_char("hello"),
        hello("John Doe"),
        first_char("hello"),
    ];
    let person = Person::new("Herman");
    let _arrayStrings = vec![alphabet_soup("hello")];
    let mut _array64 = vec![convert_days_to_age(0), cubes(3)];
    let mut _array32 = vec![
        next_integer(1),
        addition(2, 3),
        convert_minutes_to_seconds(30),
        convert_age_to_days(3),
    ];
    println!("{:?}", string_array);
    println!("{:?}", person);
    println!("{:?}", _array64);
    println!("{:?}", _array32);
    println!("Hello, world!");
}
