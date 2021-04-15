export default function isRepdigit(n) {
  if (n < 0) {
    return false;
  } else if (n === 0) {
    return true;
  } else {
    let nums = [...n.toString()];
    for (let index = 0; index < nums.length; index++) {
      if (nums[index] !== nums[0]) {
        return false;
      }
    }
    return true;
  }
}
