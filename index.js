import { readFile } from "fs";
// import * as core from "@actions/core";
import MathJax from 'mathjax'
const isAuthorized = ({ username }) => {
  console.log(username);
};
const hasPermissions = ({ permissions }) => {
  console.log(permissions);
};

const isAuthenticated = (user) => {
  console.log(user);
};
// readFile("./code_approvers.json", "utf-8", (err, data) => {
//   if (err) return err;
//   isAuthenticated.apply(null, JSON.parse(data));
// });
