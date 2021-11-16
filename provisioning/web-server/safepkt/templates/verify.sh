#!/bin/bash

set -e

function verify() {
    local package_name
    package_name="${1}"

    local output
    output="${2}"

    local smart_contract_example
    smart_contract_example="${3}"

    local proptest_cases
    proptest_cases="${4}"

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

    local panic_occurrences

    if [ -n "${proptest_cases}" ];
    then
        export PROPTEST_CASES=${proptest_cases}
        cargo verify --backend='proptest' --script=./commands.sh --tests -vvv || true
    else
        cargo verify --backend='klee' --script=./commands.sh --tests -vvvv 2> /safepkt-ink/examples/source/raw_err || true
    fi

    echo '__BEGIN_RAW_STDERR__'
    cat /safepkt-ink/examples/source/raw_err
    echo '__END_RAW_STDERR__'

    for entry_point in ./kleeout/*; do
      if [ $(echo "${entry_point}" | grep -c safe) -eq 0 ];
      then
          continue;
      fi

      echo "Tests results for "'"'"$(echo "${entry_point}" | grep safe | sed -E 's/.+::(.+)$/\1/g')"'"'"";

      # test method contains "fail" when a call is meant to panic
      if find "${entry_point}" -name "*.err" >> /dev/null 2>&1 && \
      [ $(echo "${entry_point}" | grep -c fail) -gt 0 ];
      then
        panic_occurrences=$(cat $(find ./kleeout/*submit_transaction_wallet_fails* -name *err | tail -n1 ) \
                | grep '\/panic' | grep -c "File:")

        if [ ${panic_occurrences} -gt 0 ];
        then
          echo -e '\tExpected panic occurred.'
        else
          echo -e '\tPanic should have occurred.'
        fi
        continue;
      fi

      \grep 'KLEE:' "${entry_point}/info" | sed -E 's/^/\t/g'
    done
}
verify "${1}" "${2}" ${3} ${4}
