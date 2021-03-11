export default function cardHide(card) {
  let mask = "*";
  return `${mask.repeat(card.length - 4)}${card.slice(-4)}`;
}
