export default function lengthOfLongestSubstring(mainString) {
  if (mainString.length < 1) {
    return 0;
  } else {
    let result = 0;
    let len_substring = 0;
    let substring = "";
    for (let character = 0; character < mainString.length; character++) {
      if (!substring.includes(mainString[character])) {
        substring += mainString[character];
        len_substring += 1;
      } else {
        if (len_substring > result) {
          result = len_substring;
        }
      }
      return len_substring;
    }
  }
  return new Set(mainString);
}

console.log(lengthOfLongestSubstring("abcabcbb"));
console.log(lengthOfLongestSubstring("bbbbb"));
console.log(lengthOfLongestSubstring("pwwkew"));
console.log(lengthOfLongestSubstring(""));
