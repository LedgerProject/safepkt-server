#!/bin/bash

function archive_llvm_bitcodes() {
  git clone https://github.com/thierrymarianne/contrib-rust-verification-tools rvt
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