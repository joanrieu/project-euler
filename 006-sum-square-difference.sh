#!/bin/bash -e

SQ_SUM=$((100*101/2*100*101/2))
SUM_SQ=$(seq 1 100 | awk '{ SUM += $0 * $0 } END { print SUM }')

echo $((SQ_SUM-SUM_SQ))
