export default function makeArrayConsecutive2(statues) {
  let int = 0;
  let start = Math.min(...statues);
  let target = Array(Math.max(...statues));
  statues.forEach((value) => target.splice(value - 1, 1, value));
  for (let index = start; index < target.length; index++) {
    if (target[index] === undefined) {
      int += 1;
    }
  }
  return int;
}

console.log(makeArrayConsecutive2([6, 2, 3, 8]));
