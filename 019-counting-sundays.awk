#!/usr/bin/awk -f

function tomorrow() {
    ++weekday
    if (weekday == 7+1)
        weekday = 1
    ++day
    if ( \
        (day == 28+1 && month == 2 && !leap) || \
        (day == 29+1 && month == 2 && leap) || \
        (day == 30+1 && !(month in m31)) || \
        (day == 31+1 && month in m31) \
    ) {
        day=1
        ++month
    }
    if (month == 12+1) {
        month=1
        ++year
        leap = (year % 100) ? (year % 4 == 0) : (year % 400 == 0)
    }
    leap, year, month, day, weekday
}

BEGIN {
    m31[01]
    m31[03]
    m31[05]
    m31[07]
    m31[08]
    m31[10]
    m31[12]
    
    day=1
    weekday=1
    month=1
    year=1900
    leap=0
    
    while (!(year == 1901 && month == 1 && day == 1))
        tomorrow()
    
    while (!(year == 2001 && month == 1 && day == 1)) {
        if (day == 1 && weekday == 7)
            ++sunday_first_day
        tomorrow()
    }
    print sunday_first_day
}