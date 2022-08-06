/**
 * @copyright https://leetcode.com/problems/reverse-integer/
 * @param {number} num 
 * @returns 
 */
const reverseInteger = (num) => {
	let [result, target]= [num.toString(), ''];
  for (let i = result.length; i > -1; i--) {
    if(!['0', '-'].includes(result.slice(i, i+1))) {
      target += result.slice(i, i+1)
    }
  }
  if (result[0] === '-') target = `-${target}`
return target
}

// export default reverseInteger;
console.log(reverseInteger(123)) //321
console.log(reverseInteger(-123)) // -321
console.log(reverseInteger(120)) //21