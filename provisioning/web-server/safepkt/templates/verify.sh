#!/bin/bash

function verify() {
    local output
    output="${1}"

    cp -R /ink/examples/erc20/.ink /ink/examples/source && \
    cargo verify --tests -v -o "${output}" && \
    klee --libc=klee --silent-klee-assume --warnings-only-to-file "${output}"
}
verify "${1}"