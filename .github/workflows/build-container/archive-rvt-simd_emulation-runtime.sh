#!/bin/bash

function archive_llvm_bitcodes() {
  git clone https://github.com/LedgerProject/safepkt_rust-verification-tools -b build-push-docker-images rvt
  cd rvt || exit

  tar \
  cvzf \
  "${GITHUB_WORKSPACE}/tools.tar.gz" \
  --exclude-vcs \
  --exclude='**/target/*' \
  ./cargo-verify \
  ./runtime \
  ./simd_emulation
}
archive_llvm_bitcodes