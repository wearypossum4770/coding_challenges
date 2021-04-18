pub fn mu(min:i32,max:i32)->f32{
    let a = min as f32;
    let b = max as f32;
    (a+b)/2
}
#[cfg(test)]
mod tests {
    use super::mu;
    #[test]
    fn test_mu() {
        assert_eq!(mu(64, 74), 0.1);
    }
}