pub fn reduce_array(array: &str) -> i32 {
    // let val: Vec<char> = array.chars().collect();
    let val:Vec<&str> = array.trim().split(' ').collect();
    let mut _total:i32 = 0;
    for num in &val{
    	let num: i32=num.parse().unwrap();
    	_total +=num;
    }
    return _total;
}

#[cfg(test)]
mod tests {
    use super::reduce_array;
    #[test]
    fn test_reduce_array() {
        assert_eq!(reduce_array("1 2 3 4 10 11"), 31);
    }
}
