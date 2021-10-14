FROM safepkt/rvt:llvm10

ARG UID_GID="1000:1000"
ENV UID_GID="${UID_GID}"

ARG RVT_DIR="/home/rust-verification-tools"
ENV RVT_DIR="${RVT_DIR}"

ARG LLVM_VERSION="10"

USER root

RUN apt update -y && \
  ( ! test -d "${RVT_DIR}" && mkdir "${RVT_DIR}" && \
      echo '=> Created "'"${RVT_DIR}"'" directory.')

COPY --chown=${UID_GID} ./tools.tar.gz ${RVT_DIR}/
COPY --chown=${UID_GID} ./templates/verify.sh /usr/local/bin/verify
COPY ./templates/setup-verifier.sh /usr/sbin/setup-verifier

RUN chmod +x /usr/sbin/setup-verifier && /usr/sbin/setup-verifier

COPY ./templates/unprivileged-sudoers /etc/sudoers

USER rvt

CMD ["/bin/sh"]