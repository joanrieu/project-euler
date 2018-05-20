#!/usr/bin/awk -f

function isPrime(n, _nsq, _i) {
    _nsq = sqrt(n)
    for (_i = 2; _i <= _nsq; ++_i)
        if (n % _i == 0)
            return 0
    return 1
}

BEGIN {
    n = 600851475143
    nsq = sqrt(n)
    for (i = int(nsq); i > 2; --i) {
        if (isPrime(i) && n % i == 0) {
            print i
            exit
        }
    }
}
