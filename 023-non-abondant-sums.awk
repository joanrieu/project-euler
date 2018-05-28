#!/usr/bin/awk -f

function save_multiples(n,  k) {
    for (k = n; k*n < MAX; k++)
        divs[k*n] = divs[k*n] " " n (k != n && n != 1 ? " " k : "")
}

BEGIN {
    MAX = 28124
    for (i = 1; i < MAX; i++)
        save_multiples(i)
    for (i in divs) {
        split(divs[i], k, " ")
        for (j in k)
            d[i] += k[j]
        if (d[i] <= i+0)
            delete d[i]
    }
    for (i = 1; i < MAX; i++) {
        is_sum = 0
        for (j in d)
            if (i-j in d)
                is_sum = 1
        if (i % 1000 == 0)
            print i "..."
        if (!is_sum)
            sum += i
    }
    print sum
}