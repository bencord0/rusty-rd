#!/bin/bash

BINARY="${1}"

ldd "$(which "${BINARY}")" | \
    awk '/=>/ { print $3 }' | \
    while read -r LIB; do
        echo "file ${LIB} $(readlink -f "${LIB}") 0755 0 0"
    done
