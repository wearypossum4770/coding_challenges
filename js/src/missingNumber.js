export default function missingNumber(nums){
    nums.sort((a,b)=>a-b)
    if (nums.slice(-1)[0]!==nums.length){
        return nums.length
    }
    else if (nums[0]!==0){
        return 0
    }
    for (let index=1; index<nums.length; index++){
        let expected = nums[index-1]+1
        if (nums[index]!==expected){
            return expected
        }
    }
}
console.log(missingNumber([3,0,1]))
console.log(missingNumber([0,1]))
console.log(missingNumber([9,6,4,2,3,5,7,0,1]))
console.log(missingNumber([0]))