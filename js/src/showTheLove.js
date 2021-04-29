/**
 * @copyright https://edabit.com/challenge/t5w3KeLXzs5ChWDMo
 */
export default function showTheLove(arr) {
  let share = 0;
  let smallest = Math.min(...arr);
  arr.forEach((val) => (share += val !== smallest ? val * 0.25 : 0));
  return arr.map((val) => (val === smallest ? (val += share) : (val *= 0.75)));
}
