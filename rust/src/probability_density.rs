pub fn probability_density(min: i32, max: i32) -> f32 {
    let a = min as f32;
    let b = max as f32;
   1 as f32 /(b-a)
}
pub fn normal_probability_distribution(mu:f32,sigma:f32)->f32{
    
}
#[cfg(test)]
mod tests {
    use super::probability_density;
    #[test]
    fn test_probability_density() {
        assert_eq!(probability_density(64, 74), 0.1);
    }
}
