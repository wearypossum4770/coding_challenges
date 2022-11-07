import { LinkedList } from "../index.js";
const deleteDuplicates = ({ head }) => {
  if (!head || !head.next) return false;
  let tortoise = head;
  let hare = head.next;
  while (hare && hare.next) {
    if (tortoise === hare) return true;
    tortoise = tortoise.next;
    hare = hare.next.next;
  }
  return false;
};

export default deleteDuplicates;
const data = new LinkedList();
const linkedList = [1, 1, 2].map((d) => data.add(d))[0];
console.log(deleteDuplicates(linkedList));
console.log(linkedList);
