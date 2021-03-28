pub fn change_enough(change:Vec<i31>, amount_due:f32)->bool{

}
#[cfg(test)]
mod tests {
    use super::change_enough;
    #[test]
    fn test_change_enough() {
        assert_eq!(change_enough([2, 100, 0, 0], 14.11), false);
assert_eq!(change_enough([0, 0, 20, 5], 0.75), true);
assert_eq!(change_enough([30, 40, 20, 5], 12.55), true);
assert_eq!(change_enough([10, 0, 0, 50], 13.85), false);
assert_eq!(change_enough([1, 0, 5, 219], 19.99), false);
assert_eq!(change_enough([1, 0, 2555, 219], 127.75), true);
assert_eq!(change_enough([1, 335, 0, 219], 35.21), true);    }
}
