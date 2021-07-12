import { Router } from "express";
import { UsersModel } from "../models/User.js";
const userRouter = Router();
userRouter.route("/").get((req, res) => {
  UsersModel.find()
    .then((users) => res.json(users))
    .catch((err) => res.status(400).json(`ERROR:${err}`));
});
userRouter.route("/add").post((req, res) => {
  const { username, first_name, middle_name, last_name, email, phone_number, } =
    req.body;
  const newUser = new UsersModel({ username, first_name, middle_name, last_name, email, phone_number });
  newUser
    .save()
    .then(() => res.json("User Added!"))
    .catch((err) => res.status(400).json(`Error:${err}`));
});
export default userRouter;
