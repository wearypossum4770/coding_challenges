export default function reverseCase(inputString) {
  let reversedCase = "";
  for (let index = 0; index < inputString.length; index++) {
    let character = inputString[index];
    if (character === character.toLowerCase()) {
      reversedCase += character.toUpperCase();
    } else {
      reversedCase += character.toLowerCase();
    }
  }
  return reversedCase;
}
