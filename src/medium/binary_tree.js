class Node {
  constructor(data) {
    this.data = data;
    this.left = null;
    this.right = null;
  }
}

class BinarySearchTree {
  constructor() {
    this.root = null;
  }
  insert(data) {
    var target = new Node(data);
    if (this.root === null) this.root = target;
    else this.insertNode(this.root, target);
    return this;
  }
  remove(data) {
    this.root = this.removeNode(this.root, data);
    return this;
  }
  removeNode(node, key) {
    if (node === null) return null;
    else if (key < node.data) {
      node.left = this.removeNode(node.left, key);
      return node;
    } else if (key > node.data) {
      node.right = this.removeNode(node.right, key);
      return node;
    } else {
      if (node.left === null && node.right === null) {
        node = null;
        return node;
      }
      if (node.left === null) {
        node = node.right;
        return node;
      } else if (node.right === null) {
        node = node.left;
        return node;
      }

      var aux = this.findMinNode(node.right);
      node.data = aux.data;

      node.right = this.removeNode(node.right, aux.data);
      return node;
    }
  }
  inorder(node) {
    if (node !== null) {
      this.inorder(node.left);
      console.log(node.data);
      this.inorder(node.right);
    }
  }
  postorder(node) {
    if (node !== null) {
      this.postorder(node.left);
      this.postorder(node.right);
      console.log(node.data);
    }
  }
  findMinNode(node) {
    if (node.left === null) return node;
    return this.findMinNode(node.left);
  }
  preorder(node) {
    if (node !== null) {
      console.log(node.data);
      this.preorder(node.left);
      this.preorder(node.right);
    }
  }
  insertNode(node, target) {
    if (target.data < node.data) {
      if (node.left === null) node.left = target;
      else this.insertNode(node.left, target);
    } else {
      if (node.right === null) node.right = target;
      else this.insertNode(node.right, target);
    }
    return this;
  }
  getRootNode() {
    return this.root;
  }

  search(node, data) {
    if (node === null) return null;
    else if (data < node.data) return this.search(node.left, data);
    else if (data > node.data) return this.search(node.right, data);
    return node;
  }
}

const BST = new BinarySearchTree();
BST.insert(15)
  .insert(25)
  .insert(10)
  .insert(7)
  .insert(22)
  .insert(17)
  .insert(13)
  .insert(5)
  .insert(9)
  .insert(27);
console.log(BST);
const root = BST.getRootNode();
console.log(root);
