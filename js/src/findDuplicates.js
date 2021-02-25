export default function findDuplicates(nums) {
  let obj = nums.reduce((tally, index) => {
    tally[index] = (tally[index] || 0) + 1;
    return tally;
  }, {});
  let bag = Object.entries(obj);
  let duplicates = [];
  for (let index = 0; index < bag.length; index++) {
    if (bag[index][1] > 1) {
      duplicates.push(parseInt(bag[index][0]));
    }
  }
  return duplicates.sort((a, b) => a - b);
}
