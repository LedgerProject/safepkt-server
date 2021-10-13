#!/bin/bash

function verify() {
    local package_name
    package_name="${1}"

    local output
    output="${2}"

    local smart_contract_example
    smart_contract_example="${3}"

    local cargo_home=
    cargo_home='/ink/examples/source/deps'

    if [ ! -d "${cargo_home}" ];
    then
      mkdir "${cargo_home}"
    fi

    export CARGO_HOME="${cargo_home}"

    cp -R "/ink/examples/${smart_contract_example}/.ink" /ink/examples/source && \
    sed -i 's/'"${smart_contract_example}"'/'"${package_name}"'/g' /ink/examples/source/.ink/abi_gen/Cargo.toml && \
    sed -i 's/'"${smart_contract_example}"'/'"${package_name}"'/g' /ink/examples/source/src/lib.rs && \
    cargo verify --script=./commands.sh --tests -v -o "${output}" && \
    klee --libc=klee --silent-klee-assume --warnings-only-to-file "${output}"
}
verify "${1}" "${2}" ${3}