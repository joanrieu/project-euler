#!/usr/bin/awk -f

function full_adder(    i, carry) {
    for (i = 0; i < length(a) + carry; ++i) {
        a[i] += b[i] + carry
        if (a[i] >= 10) {
            a[i] -= 10
            carry = 1
        } else {
            carry = 0
        }
    }
}

BEGIN {
    print "AWK builtin value is", 2^1000
    a[0] = 1
    for (p = 1; p <= 1000; ++p) {
        for (i in a)
            b[i] = a[i]
        full_adder()
    }
    for (i in a) {
        result = a[i] result
        sum += a[i]
    }
    print "full adder result is", result
    print "digit sum is", sum
}
