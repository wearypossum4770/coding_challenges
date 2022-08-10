pub fn add_binary(param1: &str, param2: &str) -> String {
  let binary: [u32; 30] = [1,2,4,8,16,32,64,128,256,512,1024,2048,4096,8192,16384,32768,65536,131072,262144,524288,1048576,2097152,4194304,8388608,16777216,33554432,67108864,134217728,268435456,536870912];
  String::new()
}
#[cfg(test)]
mod tests {
    use super::add_binary;
    #[test]
    fn test_add_binary() {
        assert_eq!(add_binary("11", "1"), "100");
    }
}
