import { linkedListFromArray, ListNode   } from './singly-linked-list.js'
/**
 * @copyright https://leetcode.com/problems/remove-duplicates-from-sorted-list/
 * @description
 * Leet Code #83 Remove Duplicates from Sorted List
 * Runtime 61ms Beats 66.96%of users with JavaScript
 * Memory Usage 44.54MB Beats 17.47% of users with JavaScript
 * @param {ListNode} head
 * @return {ListNode}
 */
const deleteDuplicates = (head)  => {
    const [duplicate, current] = [new Set(), new ListNode()]
    let previous =  current
    while(head) {
        if (!duplicate.has(head.val)) {
            duplicate.add(head.val)
            previous.next = head
            previous = head;
            head = head.next
        } else {
            head = head.next
        }
    }
    if (!head) previous.next = null
    return JSON.stringify(current.next)
};


// console.log(deleteDuplicates(linkedListFromArray([1,1,2,3,3])))
// console.log(deleteDuplicates(linkedListFromArray([1,1,2])))