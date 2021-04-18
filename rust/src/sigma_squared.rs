pub fn sigma_squared(min:i32,max:i32)->f32{
    let a = min as f32;
    let b = max as f32;
    let c = 12
    (b-a).pow(2)/c
}
#[cfg(test)]
mod tests {
    use super::sigma_squared;
    #[test]
    fn test_sigma_squared() {
        assert_eq!(sigma_squared(64, 74), 0.1);
    }
}