FROM safepkt/rvt_r2ct:llvm10

ARG USERNAME=rvt

USER root

RUN apt update -y

RUN apt install -y vim && \
  groupadd -g 1000 rvt && \
  useradd -s /bin/bash \
    --gid rvt \
    --uid 1000 \
    --home-dir=/home/rust-verification-tools rvt && \
  git clone https://github.com/paritytech/ink /ink && \
  cd /ink && \
  git checkout v2.1.0 && \
  chown 101.102 -R /ink

WORKDIR /ink

COPY ./templates/sudoers /etc/sudoers

USER ${USERNAME}


