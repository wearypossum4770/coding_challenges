pub fn missing_number(arr: Vec<u32>)->usize{
    let length:u32 = arr.len();
    
if arr[arr.len()-1] == arr.len(){
    arr.len()
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
