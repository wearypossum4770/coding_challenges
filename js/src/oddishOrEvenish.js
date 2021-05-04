/**
 * @copyright https://edabit.com/challenge/r6TSNwkLZ2DgsoKiH
 * @param {number} num
 * @returns {"Evenish"|"Oddish"}
 */
export default function oddishOrEvenish(num) {
  let summation = [...num.toString()].reduce(
    (accum, curr) => parseInt(accum) + parseInt(curr)
  );
  return summation % 2 === 0 ? "Evenish" : "Oddish";
}
