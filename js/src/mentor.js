export function getList(array) {
  let list1 = [];
  for (let index = 0; index < array.length; index++) {
    if (array[index] % 2 != 0) {
      list1.push(array[index]);
    }
  }
  return list1;
}

export function duplicateArray(array) {
  let array2 = [];
  for (let index = 0; index < array.length; index++) {
    array2.push(array[index]);
  }
  return array2;
}
export function SomeMethodForEfficientHandlingOfFiles() {}
export function functionSomeMethodForEfficientStorageOfFiles() {}
