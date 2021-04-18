pub mod statistics{
    pub fn mu() {}
    pub fn sigma() {}
    pub fn sigma_squared() {}
    pub fn continuous_probability() {}
    pub fn probability_density(min: i32, max: i32) -> f32 {
       1 / (max - min) as f32

    }
    
}

fn add(a: Option<i32>, b: Option<i32>) -> i32 {
    a.unwrap_or(1) + b.unwrap_or(2)
}
#[cfg(test)]
mod tests {
    use super::statistics;
    #[test]
    fn probability_density() {
        assert_eq!(probability_density(64, 74), 0.1);
    }
}
