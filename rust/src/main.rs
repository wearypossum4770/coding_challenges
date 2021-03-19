extern crate ferris_says;

use crate::addition::addition;
use crate::alphabet_soup::alphabet_soup;
use crate::convert_age_to_days::convert_age_to_days;
use crate::convert_days_to_age::convert_days_to_age;
use crate::convert_minutes_to_seconds::convert_minutes_to_seconds;
use crate::count_true::count_true;
use crate::cubes::cubes;
use ferris_says::say;
use std::io::{stdout, BufWriter};
// use crate::find_duplicates::find_duplicates;
use crate::first_char::first_char;
// use crate::fizz_buzz::fizz_buzz;
use crate::hello::hello;
use crate::name_string::name_string;
use crate::next_integer::next_integer;
// use crate::read_slice::read_slice;
// use crate::reduce_array::reduce_array;
use crate::remove_first_char::remove_first_char;
use crate::remove_spaces::remove_spaces;
// use crate::reverse_words::reverse_words;
// use crate::type_array::type_array;
// use crate::type_tuple::type_tuple;
mod addition;
mod alphabet_soup;
mod convert_age_to_days;
mod convert_days_to_age;
mod convert_minutes_to_seconds;
mod count_true;
mod cubes;
// mod find_duplicates;
mod first_char;
// mod fizz_buzz;
mod hello;
mod name_string;
mod next_integer;
// mod read_slice;
// mod reduce_array;
mod remove_first_char;
mod remove_spaces;
// mod reverse_words;
// mod type_array;
// mod type_tuple;
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
    let out = b"Hello World!";
    let width = 24;
    let mut writer = BufWriter::new(stdout());
    let string_array = vec![
        remove_spaces("hello"),
        remove_first_char("hello"),
        hello("John Doe"),
        first_char("hello"),
    ];
    let person = Person::new("Herman");
    let _array_boolean = vec![count_true(&mut [true])];
    let _array_strings = vec![alphabet_soup("hello"), name_string("Henry")];
    // let mut _array_array = vec![type_array()];
    // let mut _array_tuple = vec![type_tuple()];
    let mut _array64 = vec![convert_days_to_age(0), cubes(3)];
    let mut _array32 = vec![
        next_integer(1),
        addition(2, 3),
        convert_minutes_to_seconds(30),
        convert_age_to_days(3),
    ];
    // println!("{:?}",_array_array);
    // println!("{:?}",_array_tuple);
    println!("{:?}", string_array);
    println!("{:?}", person);
    println!("{:?}", _array64);
    println!("{:?}", _array32);
    say(out, width, &mut writer).unwrap();
}
