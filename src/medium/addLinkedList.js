class ListNode {
constructor(val, next) {
       this.val = (val===undefined ? 0 : val)
       this.next = (next===undefined ? null : next)
   }
  }
class Node{
  constructor(val){
    this.val = val
    this.next = null
  }
}
/**
 * my solution below was not usable in leet-code 
 * Runtime: 411 ms, faster than 5.03%.
 * Memory Usage: 51.4 MB, less than 5.14%.
 * @copyright https://leetcode.com/problems/add-two-numbers/
 */
class SinglyLinkedList {
  constructor() {
    this.head = null
    this.tail = null
    this.length = 0
  }
  pushAll(arr) {
    arr.forEach(n => this.push(n))
  }
  push(val)  {
    let n = new ListNode(val)
    if (!this.head) {
      this.head = n
      this.tail = this.head
    } else {
      this.tail.next = n
      this.tail = n
    }
    this.length ++
    return this
  }
}
const addTwoNumbers = (l1, l2) => {
  let linkedList = new SinglyLinkedList()
  let carry = 0
  let remainder = false
  let left = l1.head 
  let right = l2.head
  while (left != null || right!= null) {
    carry = left?.val  + right?.val + remainder
    if (carry < 10) {
      remainder = false
      linkedList.push(carry)
    }
    if (carry >= 10){
      remainder = true
      linkedList.push(carry%10)
    }
    if ((Number.isNaN(carry) && !remainder) || ((left === null || right === null) && remainder))  {
      linkedList.push(0)
    }
    left = left?.next
    right = right?.next
    if ((left === null || right === null) && remainder) {
      linkedList.push(0)
    }
    if ((left === null && right === null) && remainder) {
      linkedList.push(1)
    }
  }
  if (remainder) {
    linkedList.push(1)
  }
return linkedList
}

const linkedList1 = new SinglyLinkedList()
const linkedList2 = new SinglyLinkedList()
const linkedList3 = new SinglyLinkedList()
const linkedList4 = new SinglyLinkedList()
const linkedList5 = new SinglyLinkedList()
const linkedList6 = new SinglyLinkedList()
const linkedList7 = new SinglyLinkedList()
const linkedList8 = new SinglyLinkedList()
const linkedList9 = new SinglyLinkedList()
const linkedList10 = new SinglyLinkedList()
linkedList9.pushAll([3,2,1])
linkedList10.pushAll([2,1])
linkedList7.pushAll([4,5])
linkedList8.pushAll([3,4,5])
linkedList1.pushAll([2,4,3])
linkedList2.pushAll([5,6,4])
linkedList3.pushAll([9,9,9,9,9,9,9])
linkedList4.pushAll([9,9,9,9])
linkedList5.pushAll([0])
linkedList6.pushAll([0])
// console.log(addTwoNumbers(linkedList1, linkedList2))
// console.log(addTwoNumbers(linkedList3, linkedList4))
// console.log(addTwoNumbers(linkedList5, linkedList6))
// console.log(addTwoNumbers(linkedList7, linkedList8))
console.log(addTwoNumbers(linkedList9, linkedList10))