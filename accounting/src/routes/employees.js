import { Router } from "express";
import { EmployeeModel } from "../models/Employee.js";
const employeeRouter = Router();
employeeRouter.route("/").get((req, res) => {
  EmployeeModel.find()
    .then((employees) => res.json(employees))
    .catch((err) => res.status(400).json(`ERROR:${err}`));
});
employeeRouter.route("/add").post((req, res) => {
  const { username, first_name, middle_name, last_name, email, phone_number, } =
    req.body;
  const newUser = new EmployeeModel({ username, first_name, middle_name, last_name, email, phone_number });
  newUser
    .save()
    .then(() => res.json("Employee Added!"))
    .catch((err) => res.status(400).json(`Error:${err}`));
});
export default employeeRouter;
