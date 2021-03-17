pub fn addition(a: i32, b: i32) -> i32 {
    return a + b;
}
#[cfg(test)]
mod tests {
    use super::addition;
    #[test]
    fn test_addition() {
        assert_eq!(addition(2, 3), 5);
        assert_eq!(addition(-3, -6), -9);
        assert_eq!(addition(7, 3), 10);
        assert_eq!(addition(88, 2), 90);
    }
}
// macro_rules! addition_tests {
// 	($($name:ident: $value:expr,)*) => {
// 		$(
// #[test]
// fn $name(){
// 	let (a,b, expected) = $value;
// 	assert_eq!(expected,addition(a,b) );
// })*
// 	};
// }
// #[cfg(test)]
// addition_tests! {
//     addition_pos_5:(2,3,5),
//     addition_neg_9:(-3, -6, -9),
//     addition_pos_10:(7, 3, 10),
//     addition_pos_90:(88, 2, 90),
// }
