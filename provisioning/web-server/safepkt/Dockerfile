FROM safepkt/rvt:ink-v2.1.0

ARG UID_GID="101:102"
ENV UID_GID="${UID_GID}"

COPY ./templates/verify.sh /usr/local/bin/verify
COPY ./templates/sudoers /etc/sudoers

USER weaver

RUN sudo apt update -y && \
    sudo apt install -y less gdb

RUN find ~weaver/.cargo ~weaver/.rustup | grep -v 'share/doc' | xargs -I{} -P8 chown ${UID_GID} {} && \
    find ~weaver/.cargo ~weaver/.rustup | grep -v 'share/doc' | xargs -I{} -P8 chmod ug+rwx {} && \
    ~/.cargo/bin/rustup default nightly-2020-08-03-x86_64-unknown-linux-gnu && \
    sudo chown -R ${UID_GID} /ink && \
    sudo chgrp -R rvt /ink && \
    sudo chmod -R ug+rwx /ink && \
    sudo chmod a+x /usr/local/bin/verify

CMD ["/bin/sh"]