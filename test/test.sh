#!/bin/bash
try() {
  expected="$1"
  input="$2"

  ./main "$input" > tmp.s
  gcc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

try 0 0
try 42 42
try 3 "1+2"
try 2 "5-3"
try 21 "5+20-4"
try 10 "13 - 3"
try 41 " 12 + 34 - 5 "
try 12 "3 * 4"
try 3 "6 / 2"
try 5 "1 + 2 * 3 - 4 / 2"
try 10 "(2 * 3) + (1 * 4)"
try 17 "-3 + 20"
try 7 "10 - +(1+2)"
try 1 "4 > 3"
try 0 "7 <= 6"

echo OK

