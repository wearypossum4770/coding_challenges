/**
 * Runtime: 84 ms, Top: 88.95% .
 * Memory Usage: 40.4 MB, Top: 47.58% .
 * Leetcode problem #1528
 * https://leetcode.com/problems/shuffle-string/submissions/
 * @param {*} s 
 * @param {*} indices 
 * @returns 
 */

export default function restoreString(s, indices){
    let target = Array(indices.length)
    for (let i=0;i<indices.length;i++){
        target[indices[i]]= s[i]
    }
    return target.join('')
}

console.log(restoreString("codeleet", [4, 5, 6, 7, 0, 2, 1, 3]))
console.log(restoreString("codeleet", [4, 5, 6, 7, 0, 2, 1, 3]) == "leetcode")
console.log(restoreString("abc", [0, 1, 2]) == "abc")
console.log(restoreString("aiohn", [3, 1, 4, 2, 0]) == "nihao")
console.log(restoreString("aaiougrt", [4, 0, 2, 6, 7, 3, 1, 5]) =="arigatou")
console.log(restoreString("art", [1, 0, 2]) == "rat")
