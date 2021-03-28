export default function tetrahedralNumber(n){
    let previousValue = new Map([['value',null]])
    for (let index=1;index<n+1;index++){       
        let current =  previousValue.get('value') + (index * (index+1)) /2
        previousValue.set('value', current)
    }
    return previousValue.get('value')
}