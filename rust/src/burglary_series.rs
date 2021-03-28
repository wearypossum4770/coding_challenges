use std::collections::HashMap;
pub mod burglary_series{
    pub fn total_losses(stolen: HashMap<&str, i32>)->i32||String{
        let total: i32 = 0;
        for (item, cost) in &stolen {
            total+=cost;
        }
        if total>0{
            total
        }else {
           "Lucky you!"
        }
       }
       
    
}

#[cfg(test)]
mod tests {
    use super::burglary_series;
    #[test]
    fn test_total_losses() {
        
        assert_eq!(burglary_series::BurglarySeries::total_losses(&mut
            {tv: 30, skate: 20, stereo: 50,}), 100);
        assert_eq!(burglary_series::BurglarySeries::total_losses(&mut
            {ring: 30000, painting: 20000, bust: 1,}), 50001);
        assert_eq!(burglary_series::BurglarySeries::total_losses(&mut
            {chair: 3500,}), 3500);
        assert_eq!(burglary_series::BurglarySeries::total_losses(&mut{}), "Lucky you!");    }
}