#!/bin/bash

export EDITOR=/usr/bin/vim

function add_webserver_user() {
    groupadd --gid 33 www-data && \
    useradd --uid 33 --home /www --create-home --gid 33 --shell /sbin/nologin www-data && \
    echo 'Web server user successfully added.'
}

function add_rvt_user() {
    groupadd --gid 1000 rvt && \
    useradd --uid 1000 --home /home/rvt --create-home --gid 1000 --shell /sbin/nologin rvt && \
    echo 'RVT user successfully added.'
}

function build_rvt_docker_image() {
    cd "${RVT_DIRECTORY}" || exit
    export LLVM11=yes
    sudo -Eurvt docker/build && \
    echo 'Built RVT docker image sucessfully for LLVM 11'
    echo '/!\ Please follow instructions from https://github.com/project-oak/rust-verification-tools/issues/131#issuecomment-858638159'
    echo 'it will fix the error with message: '
    echo "ERROR - FAILED: 'llvm-link-10' terminated with exit code 1."
}
alias build-rvt-docker-image='build_rvt_docker_image'

function run_rvt_container() {
    cd "${RVT_DIRECTORY}" || exit
    # will be mounted at /source in RVT container
    export MOUNT_SOURCE=/home/rvt/klee-demo && sudo -Eurvt ./docker/run
}
alias run-rvt-container='run_rvt_container'

function remove_web_server_container() {
    docker ps -a | grep web-server | awk '{print $1}' | xargs -I{} docker rm -f {}
}

function remove_web_server_network() {
    docker network ls | grep web-server | awk '{print $1}' | xargs -I{} docker network rm {}
}

function stop_safepkt_server() {
    ps ax | grep safepkt-server | grep sudo | awk '{print $1}' | xargs -I{} kill -SIGTERM {}
}

function restart_safepkt_server() {
    stop_safepkt_server

    cd /var/www/safepkt-server && \
    sudo -urvt ./safepkt-server >> ./log/app.log 2>&1 & disown
}

function restart_web_server() {
    remove_web_server_container
    remove_web_server_network

    cd /var/services/web-server || exit
    docker-compose up -d

    # Binds the server to port 3000 of nginx container ip address
    # declared in /var/services/web-server/docker-compose.yml
    restart_safepkt_server
}
alias restart-web-server='restart_web_server'

function renew_ssl_certificates() {
    docker run -it --rm --name certbot \
    -v "/etc/letsencrypt:/etc/letsencrypt" \
    -v "/var/lib/letsencrypt:/var/lib/letsencrypt" \
    -v "/var/www/safepkt-server/public:/var/www/safepkt-server/public" \
    certbot/certbot certonly
}
alias renew-ssl-certificates='renew_ssl_certificates'

function generate_diffie_hellman_parameter() {
    openssl dhparam -out dh4096.pem 4096
}
alias generate-diffie-hellman-parameter='generate_diffie_hellman_parameter'
