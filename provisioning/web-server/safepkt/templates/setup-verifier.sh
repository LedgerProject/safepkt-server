#!/bin/bash

set -e
set -x

function setup_verifier() {
  local workdir
  workdir="$(pwd)"

  apt install -y vim less gdb htop

  git clone https://github.com/thierrymarianne/safepkt-arrayvec /safepkt-arrayvec
  cd /safepkt-arrayvec && git checkout 0.4.13

  git clone https://github.com/thierrymarianne/safepkt-rand /safepkt-rand
  cd /safepkt-rand && git checkout 0.7.4

  git clone https://github.com/paritytech/ink /safepkt-ink
  cd /safepkt-ink && git checkout v2.1.0

  git clone https://github.com/thierrymarianne/safepkt-rust-smallvec /safepkt-rust-smallvec
  cd /safepkt-rust-smallvec && git checkout v1.7.1

  chown -R ${UID_GID} /safepkt-ink
  chmod -R ug+rwx /safepkt-ink
  chmod a+x /usr/local/bin/verify

  cd "${RVT_DIR}" || exit

  tar xvzf ./tools.tar.gz
  chown -R "${UID_GID}" "${RVT_DIR}/simd_emulation" "${RVT_DIR}/runtime"
  chmod -R ug+rwx "${RVT_DIR}/simd_emulation" "${RVT_DIR}/runtime"

  rm -rf "${RVT_DIR}/cargo-verify/target"

  echo '=> rvt system user has following uid:gid '"$(id rvt)"
  find ~rvt/.cargo ~rvt/.rustup | grep -v 'share/doc' | xargs -I{} -P8 chown "${UID_GID}" {}
  find ~rvt/.cargo ~rvt/.rustup | grep -v 'share/doc' | xargs -I{} -P8 chmod ug+rwx {}
  sudo -urvt /bin/bash -xc 'export LLVM_VERSION="'${LLVM_VERSION}'" && source ~rvt/.cargo/env && which rustc && ls -lahtr `which rustc` && cd '"${RVT_DIR}"'/simd_emulation && make clean && make'
  sudo -urvt /bin/bash -xc 'export LLVM_VERSION="'${LLVM_VERSION}'" && source ~rvt/.cargo/env && cd '"${RVT_DIR}"'/runtime && make clean && make'
  sudo -urvt /bin/bash -xc "~rvt/.cargo/bin/rustup default nightly-2020-08-03 && \
    ~rvt/.cargo/bin/rustup show && source ~rvt/.cargo/env && cargo --version --verbose && cargo +nightly install --root='"${USER_HOME}"' --path='"${RVT_DIR}"'/cargo-verify"
  cp -R "${RVT_DIR}/simd_emulation" /safepkt-simd_emulation
  cp -R "${RVT_DIR}/runtime" /safepkt-runtime

  cd "${workdir}" || exit
}
setup_verifier