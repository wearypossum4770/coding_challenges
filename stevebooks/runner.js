import {readFileSync} from 'fs'
import Person from './Person.js'
// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes#mix-ins
let database = JSON.parse(readFileSync('./users/fixtures/init.json'))
function getUser(username){
    return database.filter(user=>user?.fields?.username===username).pop().fields
}
// ichigo kurosaki and orihime kurosaki are a married couple. they have one son named: kazui kurosaki
let orihime_kurosaki = new Person(getUser("orihime_kurosaki"))
let ichigo_kurosaki = new Person(getUser('ichigo_kurosaki'))
let kazui_kurosaki = new Person(getUser('kazui_kurosaki'))
ichigo_kurosaki.qualifying_children ='kazui_kurosaki'
console.log(ichigo_kurosaki.qualifying_children)

let calculatorMixin = Base => class extends Base {
    calc() { }
  };
  
  let randomizerMixin = Base => class extends Base {
    randomize() { }
  };
  class Foo { }
class Bar extends calculatorMixin(randomizerMixin(Foo)) { }
let bar = new Bar()
console.log(bar)
