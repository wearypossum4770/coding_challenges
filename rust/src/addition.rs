///
/// https://app.codesignal.com/arcade/intro/level-1/jwr339Kq6e3LQTsfa
///
pub fn addition(a: i32, b: i32) -> i32 {
    a + b
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
        assert_eq!(addition(1, 2), 3);
        assert_eq!(addition(0, 1000), 1000);
        assert_eq!(addition(2, -39), -37);
        assert_eq!(addition(99, 100), 199);
        assert_eq!(addition(-100, 100), 0);
        assert_eq!(addition(-1000, -1000), -2000);
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
