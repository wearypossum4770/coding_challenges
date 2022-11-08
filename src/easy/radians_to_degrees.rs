/// https://edabit.com/challenge/8rhnqxJFiJm5tS4G7
use std::f64::consts::PI;

pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / PI
}
#[cfg(test)]
mod tests {
    use super::radians_to_degrees;
    #[test]
    fn test_radians_to_degrees() {
        assert_eq!(radians_to_degrees(1.0), 57.29577951308232);
    }
}
