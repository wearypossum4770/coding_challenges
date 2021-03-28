export default function distributeCandies(candyType){
    let count = new Set(candyType)
    return count.size
}

console.log(distributeCandies([1,1,2,2,3,3]))