/**
 *
 * Runtime: 84 ms, faster than 69.94% of JavaScript online submissions for Pascal's Triangle.
 * Memory Usage: 42.3 MB, less than 13.93% of JavaScript online submissions for Pascal's Triangle.
 *  @param {number} numRows
 */
const pascalsTriangle = (numRows) =>  {
  const target = Array(numRows);
  for (let i = 0; i< target.length; i++) {
    target[i] = Array(i+1)
      for (let j=0; j< target[i].length; j++) {
        if (j=== 0 || j === target[i].length-1) {
          target[i][j] = 1
         }
        else{
          target[i][j] = target[i-1][j] + target[i-1][j-1]
        }
      }  
  }
  return target
}

// pascalsTriangle(10)
console.log(pascalsTriangle(10))
console.log(pascalsTriangle(5))
// console.log(pascalsTriangle(1))
// export default pascalsTriangle;













