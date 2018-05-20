#!/bin/bash -e

function numbers {
    awk '
    BEGIN {
        for (i = 999; i > 100; --i)
            for (j = 999; j > 100; --j)
                print i, j, i * j
    }
    '
}

paste \
    <(numbers) \
    <(numbers | cut -d ' ' -f 3 | rev) |
awk ' $3 == $4 { print $3 } ' |
sort -nr |
head -1
