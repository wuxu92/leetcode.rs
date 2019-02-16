#! /bin/bash

# @url https://leetcode.com/problems/transpose-file/

# Read from the file file.txt and print its transposed content to stdout.

# awk '{name=name FS $1; age=age FS $2}; END{print name"\n"age}' ./194_file.txt | sed 's/^ //'

file="file.txt"
file="194_file.txt2"
read -r cols < <(head -n 1 "$file" | awk '{print NF}')

for i in $(seq 1 $cols); do
    awk -v i=$i '{print $i}' "$file" | xargs
done


echo awk version
# awk version
awk '
{
    for (l=1; l<=NF; l++) {
        if (NR == 1) {
            grid[l] = $l;
        } else {
            grid[l] = grid[l] FS $l;
        }
    }
}
END {
    for (r=1; r<=NF; r++) { print grid[r]; }
}
' $file
