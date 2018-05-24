#/usr/bin/awk -f

function collatz(n) {
    if (n % 2)
        return 3 * n + 1
    else
        return n / 2
}

function collatz_cache(n) {
    if (n in cache)
        return cache[n]
    else
        return cache[n] = collatz(n)
    
}

function collatz_dist(n) {
    for (m = n; m > 1 || m in dist; m = collatz_cache(m))
        ++dist[n]
    if (m > 1)
        dist[n] += dist[m]
    if (dist[n] > dist[best])
        print (best = n) " has dist " dist[best]
}

BEGIN {
    for (i = 2; i <= 1e6; i++)
        collatz_dist(i)
}