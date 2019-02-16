#! /bin/bash

# @url https://leetcode.com/problems/valid-phone-numbers/

# Read from the file file.txt and output all valid phone numbers to stdout.

# while read -r line; do
#     echo $line
#     if [[ "$line" =~ '[0-9]{3}-[0-9]{3}-[0-9]{4}' ]] ; then
#         echo $line
#     fi
# done < 193_file.txt

grep -E "^[0-9]{3}-[0-9]{3}-[0-9]{4}$|^\([0-9]{3}\) [0-9]{3}-[0-9]{4}$" file.txt
