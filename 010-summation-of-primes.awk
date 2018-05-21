#!/bin/awk -f

BEGIN {
    limit=2000000
    rlimit=int(sqrt(limit))
    for (i = 2; i <= rlimit; ++i)
        for (j = i*i; j <= limit; j += i)
            sieve[j] = j
    for (i = 2; i <= limit; ++i)
        if (!(i in sieve))
            SUM += i
    print SUM
}
