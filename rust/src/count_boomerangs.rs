pub fn count_boomerangs(arr:Vec<u8>){
    
}
#[cfg(test)]
mod tests {
    use super::count_boomerangs;
    #[test]
    fn test_count_boomerangs() {
        assert_eq!(count_boomerangs([9, 5, 9, 5, 1, 1, 1]), 2);
        assert_eq!(count_boomerangs([5, 6, 6, 7, 6, 3, 9]), 1);
        assert_eq!(count_boomerangs([4, 4, 4, 9, 9, 9, 9]), 0);
        assert_eq!(count_boomerangs([5, 9, 5, 9, 5]), 3);
        assert_eq!(count_boomerangs([4, 4, 4, 8, 4, 8, 4]), 3);
        assert_eq!(count_boomerangs([2, 2, 2, 2, 2, 2, 3]), 0);
        assert_eq!(count_boomerangs([2, 2, 2, 2, 3, 2, 3]), 2);
        assert_eq!(count_boomerangs([1, 2, 1, 1, 1, 2, 1]), 2);
        assert_eq!(count_boomerangs([5, 1, 1, 1, 1, 4, 1]), 1);
        assert_eq!(count_boomerangs([3, 7, 3, 2, 1, 5, 1, 2, 2, -2, 2]), 3);
        assert_eq!(count_boomerangs([1, 7, 1, 7, 1, 7, 1]), 5);
        assert_eq!(count_boomerangs([5, 5, 5]), 0);
        
    }
}