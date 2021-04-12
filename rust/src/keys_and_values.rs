use std::collections::HashMap;
use std::ptr;
pub fn keys_and_values<'a>(obj: HashMap<&str, &str>) -> Vec<&'a str>{
    let null: *const i32 = ptr::null();
    let array: Vec<Vec<&str>> = Vec::new();
    for (key,val) in obj.iter(){
        array.push(*key);
        array.push(*val);
    }
    array
}
#[cfg(test)]
mod tests {
    use super::keys_and_values;
    #[test]
    fn test_keys_and_values() {
        let mut first = HashMap::new();
        let mut second = HashMap::new();
        first.insert("a", "1");
        first.insert("b", "2");
        first.insert("c", "3");
        second.insert("a", "Apple");
        second.insert("b", "Microsoft");
        second.insert("c", "Google");

        // let third:HashMap<&str,&str> = [{key1: true, key2: false, key3: undefined}]
        // let fourth:HashMap<&str,&str> = []
        assert_eq!(keys_and_values(first), [["a", "b", "c"], ["1", "2", "3"]]);
        assert_eq!(
            keys_and_values(second),
            [["a", "b", "c"], ["Apple", "Microsoft", "Google"]]
        );
        // assert_eq!(keys_and_values(third), [["key1", "key2", "key3"], [true, false, undefined]]);
        // assert_eq!(keys_and_values({"1": null, "2": null, "3": null}), [["1", "2", "3"], [null, null, null]]);
        // assert_eq!(keys_and_values({key1, "cat", key2, "dog", key3: null}), [["key1", "key2", "key3"], ["cat", "dog", null]]);
    }
}
