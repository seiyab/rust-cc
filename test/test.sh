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

try 0 "func main() 0"
try 3 "func main() 1+2"
try 21 "func main() 5+20-4"
try 12 "func main() 3 * 4"
try 5 "func main() 1 + 2 * 3 - 4 / 2"
try 10 "func main() (2 * 3) + (1 * 4)"
try 17 "func main() -3 + 20"
try 7 "func main() 10 - +(1+2)"
try 1 "func main() 4 > 3"
try 0 "func main() 7 <= 6"
try 0 "func main() 1 + 2 + 3 + 4 == 3 * 2"
try 1 "func main() 7 != 7 + 1"
try 1 "func main() { 1 }"
try 5 "func main() {
  let x := 5
  x
}"
try 14 "func main() {
  let a := 3
  let b := 5 * 6 - 8
  a + b / 2
}"
try 3 "func main() {
  let one := 1
  let two := 2
  let three := one + two
  three
}"
try 3 "func main() if 1 < 2 then 3 else 4"
try 20 "func main() {
  let x := 3
  if x * 2 < 5 then {
    let a := 1
    let b := 2
    a + b
  } else {
    let u := 4
    let v := 5
    u * v
  }
}"
try 3 "func main() {
  let x := 1
  let y := if x == 1 then {
      let x := 2
      x
  } else {
      let x := 3
      x
  }
  x + y
}"
try 8 "func main() {
  fib(6)
}

func fib(n) {
  if n == 0 then { 0 }
  else if n == 1 then { 1 }
  else { fib(n-1) + fib(n-2) }
}"
echo OK

