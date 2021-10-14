#!/bin/bash

set -e

function verify() {
    local package_name
    package_name="${1}"

    local output
    output="${2}"

    local smart_contract_example
    smart_contract_example="${3}"

    local cargo_home=
    cargo_home='/safepkt-ink/examples/source/deps'

    if [ ! -d "${cargo_home}" ];
    then
      mkdir "${cargo_home}"
    fi

    export CARGO_HOME="${cargo_home}"

    test -d /home/rust-verification-tools/simd_emulation && \
      rm -rf /home/rust-verification-tools/simd_emulation
    mv /safepkt-simd_emulation /home/rust-verification-tools/simd_emulation && \
      echo '=> Successfully copied restored built RVT ./simd_emulation'

    test -d /home/rust-verification-tools/runtime && \
      rm -rf /home/rust-verification-tools/runtime
    mv /safepkt-runtime /home/rust-verification-tools/runtime && \
      echo '=> Successfully copied restored built RVT ./runtime'

    cp -R "/safepkt-ink/examples/${smart_contract_example}/.ink" /safepkt-ink/examples/source && \
    sed -i 's/'"${smart_contract_example}"'/'"${package_name}"'/g' /safepkt-ink/examples/source/.ink/abi_gen/Cargo.toml && \
    sed -i 's/'"${smart_contract_example}"'/'"${package_name}"'/g' /safepkt-ink/examples/source/src/lib.rs && \
    cargo verify --script=./commands.sh --tests -v -o "${output}" && \
    klee --libc=klee --silent-klee-assume --warnings-only-to-file "${output}"
}
verify "${1}" "${2}" ${3}