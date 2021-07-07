
// Traversy media Linked List
class Node {
  constructor(data, next = null) {
    this.data = data;
    this.next = next;
  }
}

class LinkedList {
  constructor() {
    this.head = null;
    this.size = 0;
  }
  setSize = (direction = null) =>
    direction === "increase" ? (this.size += 1) : (this.size -= 1);

  // inset first node
  insertAtHead(data) {
    this.setSize("increase");

    this.head = new Node(data, this.head);
  }

  // insert last node
  insertAtTail(data) {
    this.setSize("increase");
    let node = new Node(data);
    let current;
    if (!this.head) {
      this.head = node;
    } else {
      current = this.head;
      while (current.next) {
        current = current.next;
      }
      current.next = node;
    }
  }
  //  insert at index

  // get at index

  // remove at index

  // clear list

  // print list data
  printData() {
    let current = this.head;
    while (current) {
      console.log(current.data);
      current = current.next;
    }
  }
}

const l = new LinkedList();
l.insertAtHead(100);
l.insertAtHead(200);
l.insertAtHead(300);
l.insertAtTail(20);
l.printData();


