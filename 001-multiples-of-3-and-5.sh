#!/bin/bash -e

seq 1 999 | awk '
$0 % 3 == 0 || $0 % 5 == 0 { SUM+=$0 }
END { print SUM }
'
