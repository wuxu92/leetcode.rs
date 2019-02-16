#! /bin/bash

# @url https://leetcode.com/problems/tenth-line/
# Read from the file file.txt and output the tenth line to stdout.

# head and tail
file='195_file.txt'
tail -n +10 $file | head -n 1

# awk
awk 'NR==10{print $0; exit;}' $file

#sed
sed -n 10p $file

[[ "$(wc -l $file | awk '{print $1}')" -ge 10 ]] && head -n 10 $file | tail -n 1
