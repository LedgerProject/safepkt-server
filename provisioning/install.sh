#!/bin/bash

set -e

export COMPOSE_PROJECT_NAME='safepkt'

function build_safepkt_backend_image() {
    cd "$(pwd)/provisioning/web-server" || exit

    if [ -z "${UID_GID}" ];
    then
      echo 'Please export system user uid and group gid as environment variable e.g.'
      echo 'export UID_GID="1000:1000"'

      return 1
    fi

    if [ -z "${RVT_DIRECTORY}" ] || [ ! -d "${RVT_DIRECTORY}" ];
    then
      echo 'Please export path where project-oak/rust-verification-tools were cloned to e.g.'
      echo 'export RVT_DIRECTORY="/tmp/rust-verification-tools"'

      return 1
    fi

    local workdir
    workdir="$(pwd)"

    ( cd ${RVT_DIRECTORY} && \
      tar \
        cvzf \
        ${workdir}/safepkt/tools.tar.gz \
        --exclude-vcs \
        --exclude='**/target/*' \
        ./cargo-verify \
        ./runtime \
        ./simd_emulation )

    docker-compose \
    -f ./docker-compose.yml \
    -f ./docker-compose.override.yml \
    build \
    --build-arg UID_GID="${UID_GID}" \
    --build-arg RVT_DIR="/home/rust-verification-tools" \
    --build-arg LLVM_VERSION="10" \
    --build-arg USERNAME="rvt" \
    safepkt && \
    docker tag safepkt_safepkt:latest safepkt/rvt:verifier

    test -e ./safepkt/tools.tar.gz && rm ./safepkt/tools.tar.gz
}

function clone_rvt() {
  source ./.env
  if [ -z "${RVT_DIRECTORY}" ] || [ -d "${RVT_DIRECTORY}" ];
  then
    if [ ! -e "${RVT_DIRECTORY}/.git" ];
    then
      echo 'Invalid Rust Verification Tools directory.'
      echo 'Please export a valid destination path where rvt will be cloned.'
      echo 'e.g.'
      echo 'export RVT_DIRECTORY=/tmp/rvt'

      return 1;
    fi
  fi

  if [ -e "${RVT_DIRECTORY}/.git" ];
  then
    echo 'RVT project has been cloned already.'

    return 0
  fi

  git clone https://github.com/project-oak/rust-verification-tools "${RVT_DIRECTORY}"
}

function pull_rvt_image() {
  source ./.env
  if [ "${RVT_DOCKER_IMAGE}" ] && [ -d "${RVT_DOCKER_IMAGE}" ];
  then
    echo 'Invalid container image.'
    echo 'Please export a non-empty container image for it to be pulled.'
    echo 'e.g.'
    echo 'export RVT_DOCKER_IMAGE="thierrymarianne/contrib-rvt_r2ct-llvm-11"'

    return 1;
  fi

  docker pull "${RVT_DOCKER_IMAGE}"
}

function make_runtime_simd_emulation() {
  source ./.env

  if [ -z "${RVT_DIRECTORY}" ] || [ -d "${RVT_DIRECTORY}" ];
  then
    if [ ! -e "${RVT_DIRECTORY}/.git" ];
    then
      echo 'Invalid Rust Verification Tools directory.'
      echo 'Please export a valid destination path where rvt will be cloned.'
      echo 'e.g.'
      echo 'export RVT_DIRECTORY=/tmp/rvt'

      return 1;
    fi
  fi

  if [ "${RVT_DOCKER_IMAGE}" ] && [ -d "${RVT_DOCKER_IMAGE}" ];
  then
    echo 'Invalid container image.'
    echo 'Please export a non-empty container image for it to be pulled.'
    echo 'e.g.'
    echo 'export RVT_DOCKER_IMAGE="thierrymarianne/contrib-rvt_r2ct-llvm-11"'

    return 1;
  fi

  ( docker rm -f rvt 2>&1 || echo 'no running container' ) && \
  docker run -w /home/rust-verifications-tools/runtime \
  -v "${RVT_DIRECTORY}:/home/rust-verifications-tools" \
  --name rvt "${RVT_DOCKER_IMAGE}" make

  ( docker rm -f rvt 2>&1 || echo 'no running container' ) && \
  docker run -w /home/rust-verifications-tools/simd_emulation \
  -v "${RVT_DIRECTORY}:/home/rust-verifications-tools" \
  --name rvt "${RVT_DOCKER_IMAGE}" make
}
