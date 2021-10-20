#!/bin/bash

function build() {
  cd "${GITHUB_WORKSPACE}" || exit
  cargo build --release

  mkdir "${GITHUB_WORKSPACE}/bin"

  cp "${GITHUB_WORKSPACE}/target/release/safepkt-backend" \
    "${GITHUB_WORKSPACE}"'/'"${RELEASE_NAME}"

  local cli_release_name
  cli_release_name="$(echo -n "${RELEASE_NAME}" | sed 's/backend/cli/g')"

  cp "${GITHUB_WORKSPACE}/target/release/safepkt-cli" \
    "${GITHUB_WORKSPACE}"'/'"${cli_release_name}"
}
build