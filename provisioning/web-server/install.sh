#!/bin/bash

export EDITOR=/usr/bin/vim
export SAFEPKT_DIR="${SAFEPKT_DIR:-/var/www/backend}"
export WEB_SERVER_DIR="${WEB_SERVER_DIR}:-/usr/local/nginx"

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
    if [ -z "${RVT_DIRECTORY}" ] || [ ! -d "${RVT_DIRECTORY}" ];
    then
        echo "Please export the path to rust verification tools clone."
        echo "e.g."
        echo "git clone https://github.com/project-oak/rust-verification-tools ${HOME}/rvt"
        echo "export RVT_DIRECTORY=${HOME}/rvt"

        return 1
    fi

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
    if [ -z "${RVT_DIRECTORY}" ] || [ ! -d "${RVT_DIRECTORY}" ];
    then
        echo "Please export a path to a rust verification tools project clone."
        echo "e.g."
        echo "$ git clone https://github.com/project-oak/rust-verification-tools ${HOME}/rvt"
        echo "$ export RVT_DIRECTORY=${HOME}/rvt"

        return 1
    fi

    cd "${RVT_DIRECTORY}" || exit
    # will be mounted at /source in RVT container
    export MOUNT_SOURCE=/home/rvt/klee-demo && sudo -Eurvt ./docker/run
}
alias run-rvt-container='run_rvt_container'

function remove_web_server_container() {
    docker ps -a | grep 'web-serv[e]r' | awk '{print $1}' | xargs -I{} docker rm -f {}
}

function remove_web_server_network() {
    docker network ls | grep 'web-ser[v]er' | awk '{print $1}' | xargs -I{} docker network rm {}
}

function stop_safepkt_backend() {
    ps ax | grep 'safepkt-serv[e]r' | grep sudo | awk '{print $1}' | xargs -I{} kill -SIGTERM {}
}

function restart_safepkt_backend() {
    stop_safepkt_backend

    if [ -z "${SAFEPKT_DIR}" ] || [ ! -d "${SAFEPKT_DIR}" ];
    then
        echo "Please export a path to SafePKT backend."
        echo "e.g."
        echo "$ git clone https://github.com/LedgerProject/safepkt_backend ${HOME}/safepkt-backend"
        echo "$ export SAFEPKT_DIR=${HOME}/safepkt-backend"

        return 1
    fi

    cd "${SAFEPKT_DIR}" || exit

    sudo -urvt /bin/bash -c './safepkt-backend >> ./log/app.log 2>&1' & disown
}

function restart_web_server() {
    remove_web_server_container
    remove_web_server_network

    if [ -z "${WEB_SERVER_DIR}" ] || [ ! -d "${WEB_SERVER_DIR}" ];
    then
        echo "Please export a path to the web server."
        echo "e.g."
        echo "$ git clone https://github.com/LedgerProject/safepkt_backend ${HOME}/safepkt-backend"
        echo "$ export SAFEPKT_DIR=${HOME}/safepkt-backend/provisioning/web-server"

        return 1
    fi

    cd "${WEB_SERVER_DIR}" || exit

    if [ ! -e ./docker-compose.override.yml ];
    then
        echo "The docker-compose.override.yml configuration file is missing."
        echo "It contains "
        echo " - network configuration (depends on your networks)."
        echo " - TLS certificates configuration (depends on the domain to be exposed for the backend)."
        echo " - optional basic authentication configuration."

        return 1
    fi

    docker-compose \
      -f ./docker-compose.yml \
      -f ./docker-compose.override.yml \
      up -d

    # Binds the server to port 3000 of nginx container ip address
    # declared in ${WEB_SERVER_DIR}/docker-compose.yml
    restart_safepkt_backend
}
alias restart-web-server='restart_web_server'

function renew_ssl_certificates() {
    if [ -z "${SAFEPKT_DIR}" ] || [ ! -d "${SAFEPKT_DIR}" ];
    then
        echo "Please export a path to SafePKT backend."
        echo "e.g."
        echo "$ git clone https://github.com/LedgerProject/safepkt_backend ${HOME}/safepkt-backend"
        echo "$ export SAFEPKT_DIR=${HOME}/safepkt-backend"

        return 1
    fi

    cd "${SAFEPKT_DIR}" || exit

    docker run -it --rm --name certbot \
    -v "/etc/letsencrypt:/etc/letsencrypt" \
    -v "/var/lib/letsencrypt:/var/lib/letsencrypt" \
    -v "${SAFEPKT_DIR}/public:${SAFEPKT_DIR}/public" \
    certbot/certbot certonly
}
alias renew-ssl-certificates='renew_ssl_certificates'

function generate_diffie_hellman_parameter() {
    openssl dhparam -out dh4096.pem 4096
}
alias generate-diffie-hellman-parameter='generate_diffie_hellman_parameter'
