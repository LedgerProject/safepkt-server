#!/bin/bash

function build() {
  cd "${GITHUB_WORKSPACE}" || exit
  cargo build --release

  cp "${GITHUB_WORKSPACE}/target/release/safepkt-backend" \
    "${GITHUB_WORKSPACE}"'/'"${RELEASE_NAME}" &&
    echo '=> Built '"${GITHUB_WORKSPACE}"'/'"${RELEASE_NAME}"

  cp "${GITHUB_WORKSPACE}/target/release/safepkt-cli" \
    "${GITHUB_WORKSPACE}"'/'"${CLI_RELEASE_NAME}" && \
    echo '=> Built '"${GITHUB_WORKSPACE}"'/'"${CLI_RELEASE_NAME}"
}
build