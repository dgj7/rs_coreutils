#!/bin/bash

FILES="data/states.txt
data/poem1.txt"

printf "================================================================================\n"

for f in $FILES
do
    printf -- "----------------------------------------\n"
    cargo run -- --bytes --chars --lines --words --max-line-length $f
    wc --bytes --chars --lines --words --max-line-length $f
    printf -- "----------------------------------------\n"
done

printf "================================================================================\n"
