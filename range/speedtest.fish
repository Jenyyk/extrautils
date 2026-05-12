#!/usr/bin/fish

set TEST_NUMBER 100000000

time seq $TEST_NUMBER > /dev/null
time range $TEST_NUMBER > /dev/null
