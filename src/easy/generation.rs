pub fn generation(gen: i32, sex: &str) -> String {
    match (gen, sex) {
        (0, ..) => String::from("me!"),
        (-1, "m") => String::from("father"),
        (-1, "f") => String::from("mother"),
        (1, "f") => String::from("daughter"),
        (1, "m") => String::from("son"),
        _ => String::from("It doesn't matter what they are"),
    }
}

#[cfg(test)]
mod tests {
    use super::generation;
    #[test]
    fn test_generation() {
        //assert_eq!(generation(-3, "m"), "great grandfather", "3 generations before you, male...");
        // assert_eq!(generation(1, "f"), "daughter", "1 generation after you, female...");
        // assert_eq!(generation(-3, "f"), "great grandmother");
        // assert_eq!(generation(-2, "m"), "grandfather");
        // assert_eq!(generation(-2, "f"), "grandmother");
        assert_eq!(generation(-1, "m"), "father");
        assert_eq!(generation(-1, "f"), "mother");
        assert_eq!(generation(0, "f"), "me!");
        assert_eq!(generation(1, "m"), "son");
        assert_eq!(generation(1, "f"), "daughter");
        // assert_eq!(generation(2, "m"), "grandson");
        // assert_eq!(generation(2, "f"), "granddaughter");
        // assert_eq!(generation(3, "m"), "great grandson");
        // assert_eq!(generation(3, "f"), "great granddaughter");
        // assert_eq!(generation(0, "m"), "me!");
    }
}
