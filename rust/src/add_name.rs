use std::collections::HashMap;

pub fn add_name(name: &str, score: i32) -> HashMap<&str, i32> {
    let mut person = HashMap::new();
    person.insert(name, score);
    person
}

// #[cfg(test)]
// mod tests {
//     use super::addition;
//     #[test]
//     fn test_addition() {
//         Test.assertSimilar(addName({}, "Brutus", 300), {Brutus: 300})
//         Test.assertSimilar(addName({piano: 500}, "Brutus", 400), {piano: 500, Brutus: 400})
//         Test.assertSimilar(addName({piano: 500, stereo: 300}, "Caligula", 440), {piano: 500,  stereo: 300, Caligula: 440})

//     }
// }
