export default function profitableGamble(prob, prize, pay) {
  return prob * prize > pay;
}
