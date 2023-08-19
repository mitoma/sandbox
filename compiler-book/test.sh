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

assert "8" "  4 + 2 + 4 + 3 - 5 "

# 抽象構文木を作り、乗除、優先順位、括弧に対応した
assert "47" "5+6*7"
assert "15" "5*(9-6)"
assert "4" "(3+5)/2"

# 単項演算子
assert "5" "7+-2"
assert "5" "8+(-5)"
assert "13" "8++5"

echo OK
