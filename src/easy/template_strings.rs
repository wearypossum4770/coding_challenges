/// https://edabit.com/challenge/DCmM4Eo6GQfrJoKXc
pub fn template_strings(names: [&str; 3]) -> String {
    let mut target: String = String::from("Their names were: ");
    let length = names.len();
    for (index, name) in names.iter().enumerate() {
        if index == length - 2 {
            target.push_str(name);
            target.push_str(" and ");
        } else if index == length - 1 {
            target.push_str(name);
        } else {
            target.push_str(name);
            target.push_str(", ");
        }
    }
    target + "."
}
#[cfg(test)]
mod tests {
    use super::template_strings;
    #[test]
    fn test_template_strings() {
        assert_eq!(
            template_strings(["John", "Joe", "Jack"]),
            "Their names were: John, Joe and Jack."
        );
        assert_eq!(
            template_strings(["Peter", "Pin", "Pan"]),
            "Their names were: Peter, Pin and Pan."
        );
        assert_eq!(
            template_strings(["E", "Da", "Bit"]),
            "Their names were: E, Da and Bit."
        );
        assert_eq!(
            template_strings(["Bulbasaur", "Charmander", "Squirtle"]),
            "Their names were: Bulbasaur, Charmander and Squirtle."
        );
    }
}
