#!/usr/bin/awk -f

function full_multiplier(n,    i, carry) {
    for (i = 1; i <= length(a) + carry; ++i) {
        a[i] = a[i] * n + carry
        carry = 0
        while (a[i] >= 10) {
            a[i] -= 10
            carry++
        }
    }
}

BEGIN {
    a[3]
    a[2]
    a[1] = 1
    
    n = 100
    while (n > 1)
        full_multiplier(n--)
    for (i in a)
        s += a[i]
    print s
}