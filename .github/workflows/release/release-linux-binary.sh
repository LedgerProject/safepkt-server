#!/bin/bash

function publish() {
  local binary
  binary="${1}"

  if [ ! -e "${binary}" ];
  then
      echo 'Invalid binary ('"${binary}"')'
      return 1
  fi

  local suffix
  suffix=""
  if [ $(grep 'cli' "${binary}") -gt 0 ];
  then
      suffix="-cli"
  fi

  local checksum
  checksum="$(sha256sum "${binary}" | cut -d ' ' -f 1)"

  local base_url
  base_url='https://api.github.com/repos/'"${GITHUB_REPOSITORY}"

  local upload_url
  upload_url="$(curl \
    -H 'Content-Type: application/octet-stream' \
    -H "Authorization: Bearer ${GITHUB_TOKEN}" \
    "${base_url}"/releases 2>> /dev/null | \
    jq -r '.[] | .upload_url' | \
    head -n1)"

  upload_url=${upload_url/\{?name,label\}/}

  local release_name
  release_name="$(curl \
    -H 'Content-Type: application/octet-stream' \
    -H "Authorization: Bearer ${GITHUB_TOKEN}" \
    "${base_url}"/releases 2>> /dev/null | \
    jq -r '.[] | .tag_name' | \
    head -n1)"

  curl \
    -X POST \
    --data-binary @"${binary}" \
    -H 'Content-Type: application/octet-stream' \
    -H "Authorization: Bearer ${GITHUB_TOKEN}" \
    "${upload_url}?name=${release_name}${suffix}-linux"

  curl \
    -X POST \
    --data "$checksum" \
    -H 'Content-Type: text/plain' \
    -H "Authorization: Bearer ${GITHUB_TOKEN}" \
    "${upload_url}?name=${release_name}${suffix}-linux.sha256sum"
}

CLI_RELEASE_NAME="$(echo -n "${RELEASE_NAME}" | sed -E 's/backend/cli/g')"

publish "${GITHUB_WORKSPACE}"'/'"${RELEASE_NAME}"
publish "${GITHUB_WORKSPACE}"'/'"${CLI_RELEASE_NAME}"