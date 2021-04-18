export default function numJewelsInStones(jewels, stones){
    // let counter = 0;
    // [...jewels].forEach((el) => {
    //   let regular = new RegExp(el,'g');
    //   counter += [...stones.matchAll(regular)].length;
    // })
    // return counter;
    return [...jewels].map(jewel=>[...stones.matchAll(jewel)].length).reduce((a,b)=>a+b)
    
    let count = 0
for (let index=0;index<jewels.length;index++){
    count+=[...stones.matchAll(jewels[index])].length
}
return count
}
console.log(numJewelsInStones( "aA",  "aAAbbbb"))
console.log(numJewelsInStones( "z",  "ZZ"))