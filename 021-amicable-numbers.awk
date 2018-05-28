#!/usr/bin/awk -f

function save_multiples(n,  k) {
    for (k = n; k*n < MAX; k++)
        divs[k*n] = divs[k*n] " " n (k != n && n != 1 ? " " k : "")
}

BEGIN {
    MAX = 10e3
    for (i = 1; i < MAX; i++)
        save_multiples(i)
    for (i in divs) {
        split(divs[i], k, " ")
        for (j in k)
            d[i] += k[j]
    }
    for (i in d)
        if (d[d[i]] == i && d[i] != i)
            sum += d[i]
    print sum
}