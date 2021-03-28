export default function changeEnough(change, amountDue) {
  let wallet = [
    change[0] * 0.25,
    change[1] * 0.1,
    change[2] * 0.05,
    change[3] * 0.01,
  ].reduce((a, b) => a + b);
  return wallet >= amountDue;
}
