pub fn type_tuple()(i32) {
// Tuple
// let a = (1, 1.5, true, 'a');
// let b: (i32, f64, bool, char) = (1, 1.5, true, 'a');
let mut c = (1, 1.5);
c.1 = 2;
c
}