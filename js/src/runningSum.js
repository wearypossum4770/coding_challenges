const reducer = (array) => array.reduce((accum, currVal) => accum + currVal);

export const runningSum = (nums) => {
  let results = [];
  nums.map((_, index) => {
    let end = index + 1;
    results.push(reducer(nums.slice(0, end)));
  });
  return results;
};
