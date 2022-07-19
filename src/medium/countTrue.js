/**
 * @copyright edabit - https://edabit.com/challenge/GLbuMfTtDWwDv2F73
 * @param {Array.<boolean>} arr 
 * @returns {number}
 */
const countTrue = (arr) => arr.reduce((accum, curr) => curr?accum +1:accum, 0)
export default countTrue;