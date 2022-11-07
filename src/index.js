// https://github.com/nodejs/node/issues/32316#issuecomment-606954468
import { createRequire } from "module";
import romanToInt from "./easy/romanToInt.js";
import searchInsert from "./easy/searchInsert.js";
import getVoteCount from "./easy/getVoteCount.js";
import countTrue from "./medium/countTrue.js";
import perimeter from "./medium/perimeter.js";
import numOfDigits from "./medium/numOfDigits.js";
import reverseInteger from './medium/reverseInteger.js';
import lengthOfLastWord from './easy/lengthOfLastWord.js'
import plusOne from './easy/plusOne.js'
const { default: pascalTriangleData } = await import("./large_files/pascal.json", {
  assert: {
    type: "json",
  },
});
export const require = createRequire(import.meta.url);
export const baseDirecotry = (filename) => new URL(filename, import.meta.url)
export const splitFileByLine = (fileString) => fileString.split(/\r\n|\n/)
export class Node {
  constructor(val){
    this.val = val
    this.next = null
  }

}
export class ListNode {
  constructor(data) {
    this.data = data;
    this.next = null
  }
}

export class SinglyLinkedList {
  constructor() {
    this.head = null
    this.tail=null
    this.length = 0
    return this
  }
  pushAll(arr) {
    arr.forEach(n => this.push(n))
  }
  push(val)  {
    let n = new Node(val)
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
export {
  pascalTriangleData,
  reverseInteger,
  numOfDigits,
  romanToInt,
  perimeter,
  getVoteCount,
  searchInsert,
  countTrue,
  lengthOfLastWord,
  plusOne,
};

