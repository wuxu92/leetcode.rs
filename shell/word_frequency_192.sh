# @url https://leetcode.com/problems/word-frequency/
# Read from the file words.txt and output the word frequency list to stdout.

xargs <<< "a a b" | tr ' ' '\n' | sort | uniq -c | sort -r | awk '{print $2" "$1}'
