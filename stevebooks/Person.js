import QualifyingChild from './QualifyingChild.js'
export default class Person {
    constructor({
        username="",
addresses=[],
qualifying_children=[],
dependents=[],
date_of_birth=null,
retention_only="",
do_not_contact="",
social_security_number="",
date_of_death=null,
internal_notes="",
    }){
        this._username = username
this._addresses = addresses
this._qualifying_children = qualifying_children
this._dependents = dependents
this._date_of_birth = date_of_birth===null?date_of_birth:new Date(date_of_birth)
this._retention_only = retention_only
this._do_not_contact = do_not_contact
this._social_security_number = social_security_number
this._date_of_death = date_of_death===null?date_of_death:new Date(date_of_death)
this._internal_notes = internal_notes
    }
set  qualifying_children(username){
    this._qualifying_children.push(username)
    let exists = this._qualifying_children.filter(_user=>_user.username===username)
    if (!exists){
        
    }
}
get qualifying_children (){
    return this._qualifying_children= [...new Set(this._qualifying_children)]
}
set dependents(user){
    let exists = this._dependents.filter(_user=>_user.id===user.id)
    if (!exists){
        this._dependents.push(user)
    }
}
get dependents(){
    return this._dependents
}
    markRetentionOnly(){
        return this.mark_retention_only=true
    }
}