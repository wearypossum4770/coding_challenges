pub fn oddish_or_evenish(num: i32) {
    let string = num.to_string();
    let summation = 0u32;
    for n in string.chars() {
        let val = n.to_digit(10);
        println!("{:?}", val);
    }
    println!("{:?}", summation);
}
#[cfg(test)]
mod tests {
    use super::oddish_or_evenish;
    #[test]
    fn test_oddish_or_evenish() {
        // assert_eq!(oddish_or_evenish(43), "Oddish");
        // assert_eq!(oddish_or_evenish(373), "Oddish");
        // assert_eq!(oddish_or_evenish(55551), "Oddish");
        // assert_eq!(oddish_or_evenish(694), "Oddish");
        // assert_eq!(oddish_or_evenish(4433), "Evenish");
        // assert_eq!(oddish_or_evenish(11), "Evenish");
        // assert_eq!(oddish_or_evenish(211112), "Evenish");
    }
}
