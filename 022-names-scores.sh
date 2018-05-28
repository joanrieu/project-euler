#!/bin/bash -e

curl https://projecteuler.net/project/resources/p022_names.txt | tr -d '"' | tr ',' '\n' | sort | cat -n | awk '

BEGIN {
    for (i = 1; i <= 26; i++) {
        o = sprintf("%c", 64 + i)
        print o, ord[o] = i
    }
}

// {
    split($2, c, "")
    line = 0
    for (i in c)
        line += ord[c[i]]
    line *= $1
    print sum += line
}

'