export default function removeDuplicates(nums){
    for (let index=0; index<nums.length;index++){
        while(nums[index] === nums[index+1]){
            nums.splice(index,1)
        }

    }
    return nums
}