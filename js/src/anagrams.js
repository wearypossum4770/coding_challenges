let counter = (string) =>
  string.reduce((tally, index) => {
    tally[index] = (tally[index] || 0) + 1;
    return tally;
  }, {});
export default function isAnagram(string1, string2) {
  let str1 = counter(Array.from(string1));
  let str2 = counter(Array.from(string2));
  return str1;
}

console.log(makeAnagram("cde", "abc"));
console.log(
  makeAnagram("fcrxzwscanmligyxyvym", "jxwtrhvujlmrpdoqbisbwhmgpmeoke")
);

console.log(
  isAnagram("fcrxzwscanmligyxyvym", "jxwtrhvujlmrpdoqbisbwhmgpmeoke")
);
