FROM safepkt/rvt:ink-v2.1.0

ARG UID_GID="101:102"
ENV UID_GID="${UID_GID}"

RUN sudo apt update -y && \
    sudo apt install -y less gdb

COPY ./templates/verify.sh /usr/local/bin/verify
COPY ./templates/sudoers /etc/sudoers
RUN sudo chown -R ${UID_GID} /ink && \
    sudo chgrp -R rvt /ink && \
    sudo chmod -R ug+rwx /ink && \
    sudo chmod a+x /usr/local/bin/verify

CMD ["/bin/sh"]