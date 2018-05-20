#!/bin/bash -e

awk '
BEGIN { for (i = 0; i < 4000000; ++i) print i }
' | awk '
BEGIN { N2=1 }
{ N3=N1+N2 ; if (N3 > 4000000) exit; print N3 ; N1=N2 ; N2=N3 }
' | awk '
$0 % 2 == 0 { SUM+=$0 }
END { print SUM }
'
