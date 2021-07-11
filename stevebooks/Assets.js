class Assest{
    constructor(){

    }

identify({accountType}){
    return new Map([
        [101001,"petty_cash"],
        [102001,"cash_on_hand"],
        [105251,"cash_in_bank_account"],
        [151177,  'intangible_asset'],
    ]).get(accountType)
}
}