class Size {
  constructor(width, height) {
    this.width = width;
    this.height = height;
  }
}
class Image {
  #args;
  constructor(url, width, height) {
    this.url = url;
    this.width = width;
    this.height = height;
    this.#args = Object.values(arguments);
  }
  cloneImage() {
    return new Image(...this.#args);
  }
  getUrl() {
    return this.url;
  }
  setUrl(url) {
    this.url = url;
  }
  setSize(width, height) {
    this.width = width;
    this.height = height;
  }
  getSize() {
    return new Size(this.width, this.height);
  }
  // Add methods here
}

let image1 = new Image("hackerranke.com/image1", 100, 100);
let image2 = new Image("hackerrank.com/image2", 200, 200);
let clone1 = image1.cloneImage();
// console.log(clone1)

console.log(image1);
image1.setSize(2, 3);
console.log(image1);
console.log(image1.getSize());
