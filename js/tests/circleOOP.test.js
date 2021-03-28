function round(number) {
  var factor = Math.pow(10, 5);
  return Math.round(number * factor) / factor;
}
let circo = new Circle(20);
Test.assertEquals(round(circo.getArea()), 1256.63706);
Test.assertEquals(round(circo.getPerimeter()), 125.66371);
let circo1 = new Circle(2);
Test.assertEquals(round(circo1.getArea()), 12.56637);
Test.assertEquals(round(circo1.getPerimeter()), 12.56637);
let circo2 = new Circle(4.4);
Test.assertEquals(round(circo2.getArea()), 60.82123);
Test.assertEquals(round(circo2.getPerimeter()), 27.64602);
let randomInt = round(Math.floor(Math.random() * Math.floor(200)));
//scroll down for spoilers that are hard to use

let circo3 = new Circle(randomInt);
Test.assertEquals(
  round(circo3.getArea()),
  round(Math.PI * Math.pow(randomInt, 2))
);
Test.assertEquals(round(circo3.getPerimeter()), round(2 * Math.PI * randomInt));
