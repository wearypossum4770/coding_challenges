extern crate ferris_says;
use crate::add_name::add_name;
use crate::addition::addition;
use crate::alphabet_soup::alphabet_soup;
use crate::animals::animals;
// use crate::burglary_series::burglary_series;
// use crate::century_from_year::century_from_year;
use crate::convert_age_to_days::convert_age_to_days;

use crate::convert_boolean_to_string::convert_boolean_to_string;
use crate::convert_days_to_age::convert_days_to_age;
use crate::convert_hour_minutes_to_seconds::convert_hour_minutes_to_seconds;
use crate::convert_minutes_to_seconds::convert_minutes_to_seconds;
use crate::count_true::count_true;
use crate::cubes::cubes;
use crate::football_points::football_points;
use ferris_says::say;
use std::collections::HashMap;
use std::io::{stdout, BufWriter};
// use crate::find_duplicates::find_duplicates;
use crate::find_perimeter::find_perimeter;
use crate::first_char::first_char;
// use crate::fizz_buzz::fizz_buzz;
use crate::get_first_value::get_first_value;
use crate::give_me_something::give_me_something;
use crate::hello::hello;
use crate::is_last_character_n::is_last_character_n;
use crate::less_than_100::less_than_100;
use crate::maximum_wealth::maximum_wealth;
use crate::name_string::name_string;
use crate::next_integer::next_integer;
use crate::possible_bonus::possible_bonus;
// use crate::read_slice::read_slice;
// use crate::reduce_array::reduce_array;
use crate::remove_first_char::remove_first_char;
use crate::remove_spaces::remove_spaces;
// use crate::reverse_words::reverse_words;
use crate::shift_to_right::shift_to_right;
use crate::sum_polygon::sum_polygon;
use crate::tetrahedral_number::tetrahedral_number;
// use crate::type_array::type_array;
// use crate::type_tuple::type_tuple;
use crate::using_double_ampersand::using_double_ampersand;
mod add_name;
mod addition;
mod alphabet_soup;
mod animals;
// mod burglary_series;
// mod century_from_year;
mod convert_age_to_days;
mod convert_days_to_age;
mod convert_boolean_to_string;
mod convert_hour_minutes_to_seconds;
mod convert_minutes_to_seconds;
mod count_true;
mod cubes;
mod find_perimeter;
mod football_points;
// mod find_duplicates;
mod first_char;
// mod fizz_buzz;
mod get_first_value;
mod give_me_something;
mod hello;
mod is_last_character_n;
mod less_than_100;
mod maximum_wealth;
mod name_string;
mod next_integer;
mod possible_bonus;
// mod read_slice;
// mod reduce_array;
mod remove_first_char;
mod remove_spaces;
// mod reverse_words;
mod shift_to_right;
mod sum_polygon;
mod tetrahedral_number;
// mod type_array;
// mod type_tuple;
mod using_double_ampersand;
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
    let mut score = HashMap::new();
    score.insert("piano", 500);
    score.insert("stereo", 300);
    score.insert("Caligula", 440);
    let out = b"Hello World!";
    let width = 24;
    let mut writer = BufWriter::new(stdout());
    let mut _array_objects = vec![add_name("Blue", 42)];
    let string_array = vec![
        remove_spaces("hello"),
        remove_first_char("hello"),
        hello("John Doe"),
        first_char("hello"),
    ];
    let person = Person::new("Herman");
    let _array_boolean = vec![
        using_double_ampersand(true, true),
        convert_boolean_to_string(true),
        less_than_100(3, 4),
        possible_bonus(3, 4),
        is_last_character_n("briaN"),
    ];
    let _array_strings = vec![
        give_me_something(""),
        alphabet_soup("hello"),
        name_string("Henry"),
    ];
    // let mut _array_array = vec![type_array()];
    // let mut _array_tuple = vec![type_tuple()];
    let mut _array64 = vec![
        find_perimeter(6, 7),
        maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]),
        get_first_value(vec![4, 5, 6]),
        convert_days_to_age(0),
        cubes(3),
    ];
    let mut _array_u16 = vec![
        convert_hour_minutes_to_seconds(2, 3),
        football_points(1, 2, 3),
        // century_from_year(45)
    ];
    let mut _array_u32 = vec![animals(2, 3, 4), tetrahedral_number(5)];
    let mut _array32 = vec![
        sum_polygon(3),
        count_true(&mut [true]),
        shift_to_right(2, 3),
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
    println!("{:?}", score);
    println!("{:?}", _array_u32);
    say(out, width, &mut writer).unwrap();
}
