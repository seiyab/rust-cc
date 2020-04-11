#!/bin/bash
try() {
  export CPATH=/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/
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

try 0 "return 0"
try 42 "return 42"
try 3 "return 1+2"
try 2 "return 5-3"
try 21 "return 5+20-4"
try 10 "return 13 - 3"
try 41 "return 12 + 34 - 5 "
try 12 "return 3 * 4"
try 3 "return 6 / 2"
try 5 "return 1 + 2 * 3 - 4 / 2"
try 10 "return (2 * 3) + (1 * 4)"
try 17 "return -3 + 20"
try 7 "return 10 - +(1+2)"
try 1 "return 4 > 3"
try 0 "return 7 <= 6"
try 0 "return 1 + 2 + 3 + 4 == 3 * 2"
try 1 "return 1 + 6 / 2 == 2 * 2"
try 1 "return 7 != 7 + 1"
try 5 "let x := 5
return x"
try 2 "let a := 1
let b := 3
return a + b / 3"
try 14 "let a := 3
let b := 5 * 6 - 8
return a + b / 2"
try 3 "let one := 1
let two := 2
let three := one + two
return three"

echo OK

