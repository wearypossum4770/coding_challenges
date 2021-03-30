pub fn calculate_fuel(n: f64) -> f64 {
    let fuel: f64 = n * 10.0;
    if fuel > 100.0 {
        fuel
    } else {
        100.0
    }
}
#[cfg(test)]
mod tests {
    use super::calculate_fuel;
    #[test]
    fn test_calculate_fuel() {
        assert_eq!(calculate_fuel(15.0), 150.0);
        assert_eq!(calculate_fuel(23.0), 230.0);
        assert_eq!(calculate_fuel(10.0), 100.0);
        assert_eq!(calculate_fuel(3.0), 100.0);
        assert_eq!(calculate_fuel(23.5), 235.0);
        assert_eq!(calculate_fuel(3.14), 100.0);
        assert_eq!(calculate_fuel(9.99999), 100.0);
        assert_eq!(calculate_fuel(822315322.0), 8223153220.0);
        assert_eq!(calculate_fuel(12345.6789), 123456.789);
        assert_eq!(calculate_fuel(31.41), 314.1);

        // Author: Joshua Señoron
    }
}
