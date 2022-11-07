#!/bin/sh
':' // ; cat "$0" | node --input-type=module - $@ ; exit $?
// Get-Content $PSCommandPath | Select-Object -Skip 3 | node --input-type=module - $args
// exit $LastExitCode
// <#
import * as os from 'node:os';

const {username} = os.userInfo();
console.log(`Hello ${username}!`);
// #>
