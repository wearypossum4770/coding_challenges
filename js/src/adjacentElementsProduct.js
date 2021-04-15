export default function adjacentElementsProduct(inputArray) {
  let product = inputArray[0] * inputArray[1];
  console.log(product);
  for (let index = 0; index < inputArray.length; index++) {
    let p = inputArray[index] * inputArray[index + 1];
    console.log(p);

    product = p >= product ? p : product;
  }
  return product;
}

console.log(adjacentElementsProduct([3, 6, -2, -5, 7, 3]));
