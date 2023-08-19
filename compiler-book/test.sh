#!/bin/bash
assert() {
    expected="$1"
    input="$2"

    ./mcc "$input" >tmp.s
    cc -o tmp tmp.s
    ./tmp
    actual="$?"

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual"
        exit 1
    fi
}

assert 0 0
assert 42 42

assert "3" "1+2" 
assert "5" "8-3" 
assert "5" "1+2+3+4-5" 
assert "110" "124-10-4" 
assert "20" "1024-1000-4" 

echo OK
