#!/bin/bash

function verify() {
    local output
    output="${1}"

    local cargo_home=
    cargo_home=/ink/examples/source/deps

    if [ ! -d "${cargo_home}" ];
    then
      mkdir "${cargo_home}"
    fi

    export CARGO_HOME="${cargo_home}"

    sudo chmod -R ug+rwx /ink/examples/source && \
    sudo chown -R 101:102 /ink/examples/source && \
    cp -R /ink/examples/erc20/.ink /ink/examples/source && \
    cargo verify --tests -v -o "${output}" && \
    klee --libc=klee --silent-klee-assume --warnings-only-to-file "${output}"
}
verify "${1}"