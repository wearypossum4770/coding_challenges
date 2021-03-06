function stringAnagram(dictionary, query) {
  let anagramDict = {};
  let wordDict = {};
  dictionary.forEach((word) => {
    let anagramMap = new Map();
    let letters = Array.from(word);
    letters.forEach((letter) => {
      if (anagramMap.has(letter)) {
        let char = anagramMap.get(letter);
        char += 1;
        anagramMap.set(letter, char++);
      } else {
        anagramMap.set(letter, 1);
      }
    });
    anagramDict[word] = anagramMap;
  });
  let keys = Object.values(anagramDict);
  let first = Array.from(query[0]);
  first.forEach((word) => {
    let anagramMap = new Map();
    if (anagramMap.has(letter)) {
      let char = anagramMap.get(letter);
      char += 1;
      anagramMap.set(letter, char++);
    } else {
      anagramMap.set(letter, 1);
    }
  });
  return keys;
  console.log(wordDict);
  console.log(anagramDict);
}

//console.log(stringAnagram(['heater', 'cold', 'clod', 'reheat', 'docl'], ['codl', 'heater', 'abcd']));
console.log(
  stringAnagram(
    [
      "hack",
      "a",
      "rank",
      "khac",
      "ackh",
      "kran",
      "rankhacker",
      "a",
      "ab",
      "ba",
      "stairs",
      "raits",
    ],
    ["a", "nark", "bs", "hack", "stair"]
  )
);
