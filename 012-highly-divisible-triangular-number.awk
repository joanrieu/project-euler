#!/usr/bin/awk -f

BEGIN {
    MAX=100e6
    for (i = 1; SUM <= MAX; ++i)
        divs[SUM += i]
    print SUM
    for (i = 1; i <= SUM; i++) {
        if (i < 1e3 || (i < 1e6 && i % 1e3 == 0) || (i % 1e6 == 0))
            print i
        for (j = 1; i*j <= SUM; j++)
            if (i*j in divs)
                divs[i*j]++
    }
    for (i in divs)
        if (divs[i] > MAXDIV)
            print i, MAXDIV=divs[i]
}