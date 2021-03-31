class SinglyLinkedListNode {
  constructor(nodeData) {
    this.data = nodeData;
    this.next = null;
  }
}
class SinglyLinkedList {
  constructor() {
    this.head = null;
    this.tail = null;
  }
  insertNode(nodeData) {
    const node = new SinglyLinkedListNode(nodeData);

    if (this.head == null) {
      this.head = node;
    } else {
      this.tail.next = node;
    }

    this.tail = node;
  }
}
export default function cycleDetection(head) {
  //  1. also called tortoise and hare method.
  if (!head) {
    return 0;
  }
  let tortoise = head;
  let hare = head;
  while (tortoise.next && hare.next && hare.next.next) {
    tortoise = tortoise.slow;
    hare = hare.next;
    if (tortoise === hare) {
      return 1;
    }
  }
  return 0;
}
