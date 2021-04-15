/**
 * https://leetcode.com/explore/challenge/card/april-leetcoding-challenge-2021/593/week-1-april-1st-april-7th/3696/
 */
class MyCircularQueue {
  #limit;
  constructor(size) {
    this.#limit = size;
    this.queue = Array.from(this.#limit);
  }
  Front() {
    return this.queue.slice(0)[0] ?? -1;
  }
  Rear() {
    return this.queue.slice(-1)[0] ?? -1;
  }
  enQueue(value) {}
  deQueue() {}
  isEmpty() {}
  isFull(value) {
    return this.queue.length;
    return this.queue.filter((n) => n !== undefied).length === value;
  }
}

let obj = new MyCircularQueue(3);
console.log(obj);
// console.log(obj.Front())
console.log(obj.enQueue(1)); // return True
console.log(obj);

console.log(obj.enQueue(2)); // return True
console.log(obj);

console.log(obj.enQueue(3)); // return True
// console.log(obj.enQueue(4)); // return False
console.log(obj);

// obj.Rear();     // return 3
// obj.isFull();   // return True
// obj.deQueue();  // return True
// obj.enQueue(4); // return True
// obj.Rear();     // return 4
// let  obj = new MyCircularQueue(k)
// let  param[1] = obj.enQueue(value)
// let  param[2] = obj.deQueue()
// let  param[3] = obj.Front()
// let  param[4] = obj.Rear()
// let  param[5] = obj.isEmpty()
// let  param[6] = obj.isFull()
