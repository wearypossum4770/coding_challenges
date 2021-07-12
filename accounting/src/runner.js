import Timesheet from "./components/Timesheet.js";
import mongodb from "mongodb";
let databaseName = "accountingDB";
const MongoClient = mongodb.MongoClient;
let orihime_kurosaki = {
  _id:"60ecc493009ab423506f7916"
}
function clockIn({ clock_id, timesheet_id, ...timesheet }) {
  return (
    (timesheet.endDate.getTime() - timesheet.startDate.getTime()) *
    1.6666666666667
  );
  return new Date();
}
function clockOut({ clock_id, ...timesheet }) {
  return new Date();
}
let timesheet = new Timesheet({
  timesheet_id: "00cfdcdb-29e3-43a2-b47e-f3d8424952e4",
  payrollCalendar_id: "7bc23d98-790f-4990-a159-c5c2f6c950d6",
  employee_id: "dc92aeef-c3cb-4dbe-8aaa-abfbf9e3c5c7",
  clock_id: 123,
  start_date: "2017-05-01T00:00:00",
  end_date: "2017-05-07T00:00:00",
  status: "Draft",
  totalHours: 12,
  date_modified: "2017-09-06T00:12:43",
});
let obj = clockIn(timesheet);
function obfuscate (data){}