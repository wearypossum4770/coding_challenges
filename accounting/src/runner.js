import Employee from "./components/Employee.js";
import PaymentMethod from "./components/PaymentMethod.js";
let first = new PaymentMethod({
  transaction_amount: 65,
  payment_type: "CREDIT_CARD",
});
let second = new PaymentMethod({
  transaction_amount: 62,
  payment_type: "CHECK",
});
console.log(first);
console.log(second);
