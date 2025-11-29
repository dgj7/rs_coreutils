#!/bin/bash

####
# summarize all binaries produced by this project.
####

cargo metadata --format-version 1 | jq -r '.packages[].targets[] | select(.kind | map(. == "bin") | any) | .name'
