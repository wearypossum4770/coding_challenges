import Person from './Person.js'
export default class QualifyingChild extends Person{
    constructor(args){
        super(args)
    }
    permanentlyAndTotallyDisabled(){
        this.permanently_and_totally_disabled=true
    }
}
