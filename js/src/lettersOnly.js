export default function lettersOnly(inputString) {
  return Array.from(inputString)
    .map((val) => (val.match(/[a-z]/i) ? val[0] : ""))
    .join("");
}
