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
    cp -R /safepkt-simd_emulation /home/rust-verification-tools/simd_emulation && \
      echo '=> Successfully copied LLVM bitcode generated from ./simd_emulation' >> /safepkt-ink/examples/source/verification.log

    test -d /home/rust-verification-tools/runtime && \
      rm -rf /home/rust-verification-tools/runtime
    cp -R /safepkt-runtime /home/rust-verification-tools/runtime && \
      echo '=> Successfully copied LLVM bitcode generated from RVT ./runtime' >> /safepkt-ink/examples/source/verification.log

    cp -R "/safepkt-ink/examples/${smart_contract_example}/.ink" /safepkt-ink/examples/source && \
    sed -i 's/'"${smart_contract_example}"'/'"${package_name}"'/g' /safepkt-ink/examples/source/.ink/abi_gen/Cargo.toml && \
    sed -i 's/'"${smart_contract_example}"'/'"${package_name}"'/g' /safepkt-ink/examples/source/src/lib.rs

    if [ "$(grep -c '"'"project_name"'"' /safepkt-ink/examples/source/src/lib.rs)" == "0" ];
    then
      echo "" >> /safepkt-ink/examples/source/src/lib.rs && \
      echo "// {"'"'"project_name"'"'": "'"'"${smart_contract_example}"'"'"}" >> /safepkt-ink/examples/source/src/lib.rs && \
      echo "" >> /safepkt-ink/examples/source/src/lib.rs
    fi

    cargo verify --backend=klee --script=./commands.sh --tests
}
verify "${1}" "${2}" ${3}