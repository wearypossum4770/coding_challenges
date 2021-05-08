pub fn factorial(num: u32) -> u128 {
    let mut target = 1u128;
    if num == 0 || num == 1 {
        target
    } else {
        for n in 1..=num {
            target *= n as u128;
        }
        target
    }
}
#[cfg(test)]
mod tests {
    use super::factorial;
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(6), 720);
        assert_eq!(factorial(7), 5040);
        assert_eq!(factorial(8), 40320);
        assert_eq!(factorial(9), 362880);
        assert_eq!(factorial(10), 3628800);
        assert_eq!(factorial(11), 39916800);
        assert_eq!(factorial(12), 479001600);
        assert_eq!(factorial(13), 6227020800);
        assert_eq!(factorial(14), 87178291200);
        assert_eq!(factorial(15), 1307674368000);
        assert_eq!(factorial(16), 20922789888000);
        assert_eq!(factorial(17), 355687428096000);
        assert_eq!(factorial(18), 6402373705728000);
        assert_eq!(factorial(19), 121645100408832000);
        assert_eq!(factorial(20), 2432902008176640000);
        assert_eq!(factorial(21), 51090942171709440000);
        assert_eq!(factorial(22), 1124000727777607680000);
        assert_eq!(factorial(23), 25852016738884976640000);
        assert_eq!(factorial(24), 620448401733239439360000);
        assert_eq!(factorial(25), 15511210043330985984000000);
        assert_eq!(factorial(26), 403291461126605635584000000);
        assert_eq!(factorial(27), 10888869450418352160768000000);
        assert_eq!(factorial(28), 304888344611713860501504000000);
        assert_eq!(factorial(29), 8841761993739701954543616000000);
        assert_eq!(factorial(30), 265252859812191058636308480000000);
        assert_eq!(factorial(31), 8222838654177922817725562880000000);
        assert_eq!(factorial(32), 263130836933693530167218012160000000);
        assert_eq!(factorial(33), 8683317618811886495518194401280000000);
        assert_eq!(factorial(34), 295232799039604140847618609643520000000);
    }
}
