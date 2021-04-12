pub fn missing_number(arr: Vec<u32>)->u32{
    let length:u32 = arr.len().try_into().unwrap();
    
if arr[arr.len()-1] == length{
    length
}else{
//  if arr[0] != 0{
    0
}
}
#[cfg(test)]
mod tests {
    use super::missing_number;
    #[test]
    fn test_missing_number() {
        assert_eq!(missing_number(vec![3,0,1]),2 );
        assert_eq!(missing_number(vec![0,1]), 2);
        assert_eq!(missing_number(vec![9,6,4,2,3,5,7,0,1]), 8);
        assert_eq!(missing_number(vec![0]), 1);    }
}
