#!/usr/bin/awk -f

function isPrime(n, _nsq, _i) {
    _nsq = sqrt(n)
    for (_i = 2; _i <= _nsq; ++_i)
        if (n % _i == 0)
            return 0
    return 1
}

BEGIN {
    n = 0
    for (i = 2; n < 10001; ++i) {
        if (isPrime(i)) {
            print i
            ++n
        }
    }
}
