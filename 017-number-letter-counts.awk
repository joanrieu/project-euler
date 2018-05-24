#!/usr/bin/awk -f

BEGIN {
    say[1] = "one"
    say[2] = "two"
    say[3] = "three"
    say[4] = "four"
    say[5] = "five"
    say[6] = "six"
    say[7] = "seven"
    say[8] = "eight"
    say[9] = "nine"
    say[10] = "ten"
    say[11] = "eleven"
    say[12] = "twelve"
    say[13] = "thirteen"
    say[14] = "fourteen"
    say[15] = "fifteen"
    say[16] = "sixteen"
    say[17] = "seventeen"
    say[18] = "eighteen"
    say[19] = "nineteen"
    say[20] = "twenty"
    say[30] = "thirty"
    say[40] = "forty"
    say[50] = "fifty"
    say[60] = "sixty"
    say[70] = "seventy"
    say[80] = "eighty"
    say[90] = "ninety"
    
    for (i = 1; i <= 1000; ++i) {
        digit = i % 10
        tens = i % 100 - digit
        hundreds = i % 1000 - tens - digit
        res = ""
        if (i == 1000)
            res = res "one thousand"
        if (hundreds)
            res = res say[hundreds / 100] " hundred"
        if (hundreds && (tens || digit))
            res = res " and "
        if (i % 100 > 20) {
            if (digit)
                res = res say[tens] "-" say[digit]
            else
                res = res say[tens]
        } else {
            res = res say[i % 100]
        }
        print res
        bigres = bigres " " res
    }
    
    gsub(/ |-/, "", bigres)
    print length(bigres)
}
