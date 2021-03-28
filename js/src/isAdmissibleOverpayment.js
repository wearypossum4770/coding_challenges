let notes = ["10.0% higher than in-store", 
"5.0% lower than in-store", 
"Same as in-store"]
let prices = [110, 95, 70]
let x = 5
export default function isAdmissibleOverpayment(prices, notes, x){

    let target = 0
    let parsedNotes = notes.map(note=>note.split(/\W+/gim))
    parsedNotes.forEach(note=>{
    let coupon  = parseInt(note[0])
    if (Number.isInteger(coupon)){
        let isHigher = note.includes('higher')
        if (isHigher){
            target+=coupon
        }else {
            target-=coupon
        }    
    }
    })
    return target <= x
}

console.log(isAdmissibleOverpayment(prices, notes, x))