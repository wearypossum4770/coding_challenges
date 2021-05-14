pub fn fizz_buzz(num: u32) {
    for i in 1..=num {
        match (i % 3, i % 5) {
            (0, 0) => println!("Fizzbuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", i),
        }
    }
}
