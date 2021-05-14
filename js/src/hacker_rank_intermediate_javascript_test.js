'use strict';

const fs = require('fs');

process.stdin.resume();
process.stdin.setEncoding("ascii");
let inputString = "";
let currentLine = 0;

process.stdin.on("data", function (chunk) {
    inputString += chunk;
});
process.stdin.on("end", function () {
    inputString = inputString.split('\n');
    main();
});

function readLine() {
  return inputString[currentLine++];
}

function Activity(amount) {}
Activity.prototype.setAmount = function(value){
    if(value<=0){
        return false
    }else {
        this.amount=value
        return true
    }
}
Activity.prototype.getAmount = function(){return this.amount}
function Payment(amount, receiver) {}

function Refund(amount, sender) {}


// function readLine() {
//   return inputString[currentLine++];
// }

class Size {
    constructor(width, height) {
        this.width = width;
        this.height = height;
    }
}

class Image {
    constructor(url,size){
        this.url = url
        this.size=size
    }
    cloneImage(){
        return new Image(this.constructor)
    }
    getUrl(){
        return this.url
    }
    setUrl(url){
        this.url=url
    }
    setSize(size)
    // Add methods here
}

