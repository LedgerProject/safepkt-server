FROM safepkt/rvt:llvm10

USER root

COPY ./templates/verify.sh /usr/local/bin/verify
COPY ./templates/sudoers /etc/sudoers

ARG UID_GID="101:102"
ENV UID_GID="${UID_GID}"

RUN apt update -y && \
  apt install -y vim less gdb htop && \
  git clone https://github.com/thierrymarianne/safepkt-arrayvec /safepkt-arrayvec && \
  cd /safepkt-arrayvec && git checkout 0.4.13 && \
  git clone https://github.com/thierrymarianne/safepkt-rand /safepkt-rand && \
  cd /safepkt-rand && git checkout 0.7.4 && \
  git clone https://github.com/paritytech/ink /safepkt-ink && \
  cd /safepkt-ink && git checkout v2.1.0 && \
  git clone https://github.com/thierrymarianne/safepkt-rust-smallvec /safepkt-rust-smallvec && \
  cd /safepkt-rust-smallvec && git checkout v1.7.1 && \
  find ~rvt/.cargo ~rvt/.rustup | grep -v 'share/doc' | xargs -I{} -P8 chown ${UID_GID} {} && \
  find ~rvt/.cargo ~rvt/.rustup | grep -v 'share/doc' | xargs -I{} -P8 chmod ug+rwx {} && \
  sudo -urvt ~rvt/.cargo/bin/rustup default nightly-2020-08-03-x86_64-unknown-linux-gnu && \
  chown -R ${UID_GID} /safepkt-ink && \
  chmod -R ug+rwx /safepkt-ink && \
  chmod a+x /usr/local/bin/verify

COPY ./templates/unprivileged-sudoers /etc/sudoers
USER rvt

CMD ["/bin/sh"]