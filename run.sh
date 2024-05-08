#!/bin/bash

FILES="data/states.txt
data/poem1.txt
data/40k.txt
data/empty.txt
data/single-line-no-newline.txt
data/single-line-too-much-whitespace.txt"

printf "================================================================================\n"
printf "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n"
cargo build || exit
printf "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n"
printf "lines, words, bytes, chars, max\n"
printf " ---------------------------------------\n"

for f in $FILES
do
    RWCO=$(target/debug/rwc --bytes --chars --lines --words --max-line-length $f)
    WCO=$(wc --bytes --chars --lines --words --max-line-length $f)

    printf " RWC|$RWCO\n"
    printf "  WC|$WCO\n"
    printf " ---------------------------------------\n"
done

printf "================================================================================\n"
