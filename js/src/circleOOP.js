class Circle {
  constructor(radius, ...args) {
    this.radius = radius;
  }

  getArea() {
    return Math.PI * Math.pow(this.radius, 2);
  }
  getPerimeter() {
    return this.radius * 2 * Math.PI;
  }
}
