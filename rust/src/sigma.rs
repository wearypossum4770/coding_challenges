pub fn sigma(min:i32,max:i32)->f32{
    let a = min as f32;
    let b = max as f32;
    let c = 12
    (b-a)/c.sqrt()
}
#[cfg(test)]
mod tests {
    use super::sigma;
    #[test]
    fn test_sigma() {
        assert_eq!(sigma(64, 74), 0.1);
    }
}