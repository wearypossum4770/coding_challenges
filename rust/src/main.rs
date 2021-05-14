// jankines8@gmail.com
// bpb-dLwK6Nwb75k
extern crate ferris_says;
use crate::add_name::add_name;
use crate::addition::addition;
// use crate::adjacent_elements_product::adjacent_elements_product;
use crate::alphabet_soup::alphabet_soup;
use crate::animals::animals;
use crate::array_of_multiples::array_of_multiples;
use crate::ascending_descending_none::ascending_descending_none;
use crate::date_challenge::date_challenge;
use ferris_says::say;
use std::collections::HashMap;
use std::io::{stdout, BufWriter};
// use crate::burglary_series::burglary_series;
use crate::calculate_fuel::calculate_fuel;
// use crate::century_from_year::century_from_year;
use crate::check_if_pangram::check_if_pangram;
use crate::check_palindrome::check_palindrome;
use crate::convert_age_to_days::convert_age_to_days;
use crate::convert_boolean_to_string::convert_boolean_to_string;
use crate::convert_days_to_age::convert_days_to_age;
use crate::convert_hour_minutes_to_seconds::convert_hour_minutes_to_seconds;
use crate::convert_minutes_to_seconds::convert_minutes_to_seconds;
use crate::count_matches::count_matches;
use crate::count_true::count_true;
use crate::cubes::cubes;
use crate::football_points::football_points;
// use crate::find_duplicates::find_duplicates;
use crate::find_perimeter::find_perimeter;
use crate::first_char::first_char;
// use crate::fizz_buzz::fizz_buzz;
use crate::frames_per_second::frames_per_second;
use crate::get_first_value::get_first_value;
use crate::give_me_something::give_me_something;
use crate::hello::hello;
use crate::int_within_bounds::int_within_bounds;
use crate::is_even::is_even;
use crate::is_last_character_n::is_last_character_n;
use crate::is_rep_digit::is_rep_digit;
use crate::is_same_number::is_same_number;
// use crate::keys_and_values::keys_and_values;
use crate::less_than_100::less_than_100;
use crate::letter_combinations::letter_combinations;
use crate::make_pair::make_pair;
use crate::makes_ten::makes_ten;
use crate::maximum_wealth::maximum_wealth;
use crate::min_operations::min_operations;
// use crate::missing_number::missing_number;
use crate::name_string::name_string;
use crate::next_integer::next_integer;
use crate::nth_even::nth_even;
use crate::oddish_or_evenish::oddish_or_evenish;
use crate::possible_bonus::possible_bonus;
use crate::profitable_gamble::profitable_gamble;
// use crate::read_slice::read_slice;
// use crate::reduce_array::reduce_array;
use crate::reduce_to_zero_steps::reduce_to_zero_steps;
use crate::remove_first_char::remove_first_char;
use crate::remove_spaces::remove_spaces;
use crate::return_negative::return_negative;
// use crate::reverse_words::reverse_words;
// use crate::reverse_array::reverse_array;
use crate::seven_boom::seven_boom;
use crate::shift_to_right::shift_to_right;
// use crate::shuffle_string::shuffle_string;
use crate::smaller_numbers_than_current::smaller_numbers_than_current;
use crate::subtract_product_and_sum::subtract_product_and_sum;
use crate::sum_polygon::sum_polygon;
use crate::tetrahedral_number::tetrahedral_number;
use crate::triangular_numbers::triangular_numbers;
// use crate::type_array::type_array;
// use crate::type_tuple::type_tuple;
use crate::using_double_ampersand::using_double_ampersand;
use crate::years_in_one_house::years_in_one_house;
mod add_name;
mod addition;
// mod adjacent_elements_product;
mod alphabet_soup;
mod animals;
mod array_of_multiples;
mod ascending_descending_none;
// mod burglary_series;
mod calculate_fuel;
// mod century_from_year;
mod check_if_pangram;
mod check_palindrome;
mod convert_age_to_days;
mod convert_boolean_to_string;
mod convert_days_to_age;
mod convert_hour_minutes_to_seconds;
mod convert_minutes_to_seconds;
mod count_matches;
mod count_true;
mod cubes;
mod date_challenge;
mod find_perimeter;
mod football_points;
// mod find_duplicates;
mod first_char;
// mod fizz_buzz;
mod frames_per_second;
mod get_first_value;
mod give_me_something;
mod hello;
mod int_within_bounds;
mod is_even;
mod is_last_character_n;
mod is_rep_digit;
mod is_same_number;
// mod keys_and_values;
mod less_than_100;
mod letter_combinations;
mod make_pair;
mod makes_ten;
mod maximum_wealth;
mod min_operations;
// mod missing_number;
mod name_string;
mod next_integer;
mod nth_even;
mod oddish_or_evenish;
mod possible_bonus;
mod profitable_gamble;
// mod read_slice;
// mod reduce_array;
mod reduce_to_zero_steps;
mod remove_first_char;
mod remove_spaces;
mod return_negative;
// mod reverse_array;
// mod reverse_words;
mod seven_boom;
mod shift_to_right;
// mod shuffle_string;
mod smaller_numbers_than_current;
mod subtract_product_and_sum;
mod sum_polygon;
mod tetrahedral_number;
mod triangular_numbers;
// mod type_array;
// mod type_tuple;
mod using_double_ampersand;
mod years_in_one_house;
#[macro_use]
extern crate maplit;
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
    println!(
        "{:?}",
        count_matches(
            &[
                ["phone", "blue", "pixel"],
                ["computer", "silver", "phone"],
                ["phone", "gold", "iphone"]
            ],
            "type",
            "phone"
        )
    );
    let map = hashmap! {
        "a" => 1,
        "b" => 2,
    };
    let mut second = HashMap::new();
    second.insert("a", "Apple");
    second.insert("b", "Microsoft");
    second.insert("c", "Google");
    date_challenge();
    let mut score = HashMap::new();
    score.insert("piano", 500);
    score.insert("stereo", 300);
    score.insert("Caligula", 440);
    let out = b"Hello World!";
    let width = 24;
    let mut writer = BufWriter::new(stdout());
    let mut _array_i32 = vec![
        triangular_numbers(2),
        min_operations(3),
        nth_even(0),
        return_negative(1),
        // adjacent_elements_product(&[3, 6, -2, -5, 7, 3]),
        sum_polygon(3),
        subtract_product_and_sum(32),
        count_true(&mut [true]),
        shift_to_right(2, 3),
        next_integer(1),
        addition(2, 3),
        convert_minutes_to_seconds(30),
        convert_age_to_days(3),
    ];
    let mut _array_i64 = vec![
        find_perimeter(6, 7),
        maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]),
        get_first_value(vec![4, 5, 6]),
        convert_days_to_age(0),
        cubes(3),
    ];
    // let mut _array_f32 = vec![];
    let mut _array_f64 = vec![calculate_fuel(3.0)];
    let mut _array_u8 = vec![years_in_one_house(30, 0)];
    let mut _array_u16 = vec![
        convert_hour_minutes_to_seconds(2, 3),
        football_points(1, 2, 3),
        // century_from_year(450),
    ];
    let mut _array_u32 = vec![
        // missing_number(vec![1]),
        frames_per_second(1, 2),
        animals(2, 3, 4),
        tetrahedral_number(5),
    ];
    let mut _array_i32_size_2 = [make_pair(1, 2)];
    let mut _array_u64 = vec![reduce_to_zero_steps(14)];
    let mut _array_i32_vectors = vec![ascending_descending_none(&[4, 3, 2, 1], "Asc")];
    let mut _array_u32_vectors = vec![
        array_of_multiples(2, 3),
        // reverse_array(&[3, 2, 1]),
        // keys_and_values(second)
    ];
    let moderator_scores = (
        "toxic",
        "severe_toxic",
        "obscence",
        "insult",
        "identity_hate",
    );
    let mut _array_objects = vec![add_name("Blue", 42)];
    let string_array = vec![
        remove_spaces("hello"),
        remove_first_char("hello"),
        hello("John Doe"),
        convert_boolean_to_string(true),
        first_char("hello"),
    ];
    let person = Person::new("Herman");
    let _array_boolean = vec![
        profitable_gamble(1.0, 2, 1.0),
        check_palindrome("A"),
        is_even(2),
        check_if_pangram("a"),
        is_rep_digit(22),
        makes_ten(10, 1),
        int_within_bounds(1, 2, 3),
        is_same_number(2, 3),
        using_double_ampersand(true, true),
        less_than_100(3, 4),
        possible_bonus(3, 4),
        is_last_character_n("briaN"),
    ];
    let _array_strings = vec![
        // shuffle_string("art", &[1, 0, 2]),
        give_me_something(""),
        seven_boom(vec![2]),
        alphabet_soup("hello"),
        name_string("Henry"),
    ];
    // let mut _array_array = vec![type_array()];
    // let mut _array_tuple = vec![type_tuple()];
    println!("{:?}", person);
    println!("{:?}", score);
    println!("{:?}", moderator_scores);
    // println!("{:?}",_array_array);
    // println!("{:?}",_array_tuple);
    println!("{:?}", string_array);
    println!("{:?}", _array_i64);
    println!("{:?}", _array_i32);
    println!("{:?}", _array_u32);
    println!("{:?}", _array_u64);
    println!("{:?}", _array_u8);
    println!("{:?}", _array_i32_vectors);
    println!("{:?}", _array_u32_vectors);
    println!("{:?}", map);
    println!("{:?}", _array_i32_size_2);
    println!("{:?}", letter_combinations("2"));
    println!("{:?}", smaller_numbers_than_current(&[8, 1, 2, 2, 3]));
    println!("{:?}", oddish_or_evenish(123));
    say(out, width, &mut writer).unwrap();
}
