import { readFile } from 'fs'
import * as core from '@actions/core';

const isAuthenticated = (user) => {
console.log(user)
}
readFile('./code_approvers.json', 'utf-8', (err, data) => {
  if (err) return err
  isAuthenticated.apply(null, JSON.parse(data))
})