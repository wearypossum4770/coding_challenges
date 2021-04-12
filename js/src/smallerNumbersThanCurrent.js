export default function smallerNumbersThanCurrent(nums) {
  return nums.map((currentValue, _, array) => {
    let value = 0;
    for (let index = 0; index < array.length; index++) {
      if (array[index] < currentValue) {
        value++;
      }
    }
    return value;
  });
}
