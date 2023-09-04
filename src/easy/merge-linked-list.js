/**
 * 
 * @typedef {Object} ListNode
 * @property { number | 0 } val
 * @property { number | null } next
 */

/**
 * @description
 * Definition for singly-linked list.
 * function ListNode(val, next) { return Object.assign(this, { val: val ?? 0, next: next??null})}
 * @copyright https://leetcode.com/problems/merge-two-sorted-lists/ 
 * @param { ListNode } list1
 * @param { ListNode } list2
 * @return { ListNode }
 */

const mergeTwoLists = (list1, list2) => {
    const current = new ListNode()
    let previous = current
    while(list1 && list2) {
        if (list1.val <= list2.val) {
            previous.next = list1
            previous = list1;
            list1 = list1.next
        } else {
            previous.next = list2
            previous = list2
            list2=list2.next
        }
    }
    if (!list1) previous.next =list2
    if (!list2) previous.next = list1
    return current.next
}



export const merge = (param1, param2) => mergeTwoLists(linkedListFromArray(param1), linkedListFromArray(param2))