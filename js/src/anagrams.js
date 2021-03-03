function makeAnagram(a, b) {
  let alpha = Array.from(a).sort();
  let beta = Array.from(b).sort();
  let target = 0;
  let c = "";
  let d = "";
  alpha.forEach((char) => {
    if (!beta.includes(char)) {
      target += 1;
    } else {
      c += char;
    }
  });
  beta.forEach((char) => {
    if (!alpha.includes(char)) {
      target += 1;
    } else {
      d += char;
    }
  });
  console.log(c);
  console.log(d);
  return target;
}

console.log(makeAnagram("cde", "abc"));
console.log(
  makeAnagram("fcrxzwscanmligyxyvym", "jxwtrhvujlmrpdoqbisbwhmgpmeoke")
);
