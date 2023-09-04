export default function ListNode(val, next) {
    return Object.assign(this, { val: val ?? 0, next: next??null})
}


const linkedListFromArray = (arr) => arr.reduceRight((next, val) => ({val, next}), null);
export { ListNode,linkedListFromArray }
// console.log(merge(linkedListFromArray([1,2,4]), linkedListFromArray([1,3,4])))

