#!/bin/bash

FILES="data/states.txt
data/poem1.txt
data/40k.txt
data/empty.txt
data/single-line-no-newline.txt"

printf "================================================================================\n"
printf "lines, words, bytes, chars, max\n"

for f in $FILES
do
    printf -- "----------------------------------------\n"
    cargo run -- --bytes --chars --lines --words --max-line-length $f
    wc --bytes --chars --lines --words --max-line-length $f
    printf -- "----------------------------------------\n"
done

printf "================================================================================\n"
