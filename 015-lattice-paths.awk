#!/usr/bin/awk -f

function factorial(n) {
    if (n > 1)
        return n * factorial(n-1)
    else
        return 1
}

# https://en.wikipedia.org/wiki/Permutation#Permutations_of_multisets
function paths(size) {
    return factorial(size + size) / (factorial(size) * factorial(size))
}

BEGIN {
    print paths(20)
}