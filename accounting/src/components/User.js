import mongoose from "mongoose";

const Schema = mongoose.Schema;
const { Date, ObjectId, String, Number } = Schema;
const User = new Schema({
  employee_id: ObjectId,
  first_name: String,
  middle_name: String,
  last_name: String,
  date_of_birth: String,
  email: String,
  phone_number: String,
  department: String,
  salaries_and_wages: String,
  position: String,
  start_date: Date,
  end_date: Date,
  date_created: Date,
  date_modified: Date,
});
