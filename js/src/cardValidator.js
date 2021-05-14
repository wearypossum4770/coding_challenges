let regex = new Map([
  [/"^4"/, "visa"],
  [/"^(34|37)"/, "amex"],
  [/"^5[1-5]"/, "mastercard"],
  [/"^6011"/, "discover"],
  [/'^9792'/, "troy"],
]);
console.log(regex);

export default function cardValidator() {
  // https://codepen.io/JavaScriptJunkie/pen/YzzNGeR
  (this.cardName = ""),
    (this.cardNumber = ""),
    (this.cardMonth = ""),
    (this.cardYear = ""),
    (this.cardCvv = ""),
    (this.minCardYear = new Date().getFullYear()),
    (this.amexCardMask = "#### ###### #####"),
    (this.otherCardMask = "#### #### #### ####"),
    (this.cardNumberTemp = ""),
    (this.isCardFlipped = false),
    (this.focusElementStyle = null),
    (this.isInputFocused = false);
}
function Person() {
  this.name = "john";
  this.age = 23;
}
const person = new Person();
let card = new cardValidator();
console.log(card);
console.log(person);
