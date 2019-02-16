# @url https://leetcode.com/problems/word-frequency/
# Read from the file words.txt and output the word frequency list to stdout.

# both pass
# xargs < words.txt | tr ' ' '\n' | sort | uniq -c | sort -r | awk '{print $2" "$1}'
tr ' ' '\n' < words.txt | sort | uniq -c | sort -nr | awk 'NF>1{print $2" "$1}'
