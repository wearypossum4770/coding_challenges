pub fn fibonacci(n:u32)->u128{
	let five = 5.0f64;
	let root = five.sqrt();
	  (
	  Math.pow(1 + root, n) - Math.pow(1 - root, n)) / (Math.pow(2, n) * root)
	
}
#[cfg(test)]
mod tests {
    use super::fibonacci;
    #[test]
    fn test_fibonacci() {

 assert_eq!(fibonacci(0),0);
assert_eq!(fibonacci(1),1);
assert_eq!(fibonacci(2),1);
assert_eq!(fibonacci(3),2);
assert_eq!(fibonacci(4),3);
assert_eq!(fibonacci(5),5);
assert_eq!(fibonacci(6),8);
assert_eq!(fibonacci(7),13);
assert_eq!(fibonacci(8),21);
assert_eq!(fibonacci(9),34);
assert_eq!(fibonacci(10),55);
assert_eq!(fibonacci(11),89);
assert_eq!(fibonacci(12),144);
assert_eq!(fibonacci(13),233);
assert_eq!(fibonacci(14),377);
assert_eq!(fibonacci(15),610);
assert_eq!(fibonacci(16),987);
assert_eq!(fibonacci(17),1597);
assert_eq!(fibonacci(18),2584);
assert_eq!(fibonacci(19),4181);
assert_eq!(fibonacci(20),6765);
assert_eq!(fibonacci(21),10946);
assert_eq!(fibonacci(22),17711);
assert_eq!(fibonacci(23),28657);
assert_eq!(fibonacci(24),46368);
assert_eq!(fibonacci(25),75025);
assert_eq!(fibonacci(26),121393);
assert_eq!(fibonacci(27),196418);
assert_eq!(fibonacci(28),317811);
assert_eq!(fibonacci(29),514229);
assert_eq!(fibonacci(30),832040);
    }
}
