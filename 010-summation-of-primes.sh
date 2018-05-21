#!/bin/bash -e

seq 3 2000000 | awk '
BEGIN {
    primes[0] = 2
    SUM += 2
}
{
    for (i in primes)
        if ($0 % primes[i] == 0)
            next
    primes[length(primes)] = $0
    SUM += $0
    print "+", $0, "=", SUM
}
'