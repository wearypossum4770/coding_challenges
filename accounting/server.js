import mongoose from "mongoose";
import express from "express";
import mongodb from "mongodb";
import cors from "cors";
import userRouter from "./src/routes/users.js";
import employeeRouter from './src/routes/employees.js'
const MongoClient = mongodb.MongoClient;
const PORT = 3003;
const app = express();
app.use(express.json());
app.use(express.urlencoded({ extended: true }));
app.use(cors());
app.get("/", (req, res) => res.json("HELLO"));
app.use("/users", userRouter);
app.use("/employees", employeeRouter);
const databaseName = "accountingDB";
mongoose.connect(`mongodb://localhost/${databaseName}`, {
  useNewUrlParser: true,
  useUnifiedTopology: true,
  useFindAndModify: false,
  useCreateIndex: true,
});
const connection = mongoose.connection;
connection.once("open", () => {
  console.log("MongoDB database connection established successfully");
});
app.listen(PORT, () => console.log(`Server started on port:${PORT}`));
