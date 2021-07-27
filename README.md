# SafePKT backend

This project is implemented in the context of the European [NGI LEDGER program](https://ledger-3rd-open-call.fundingbox.com/).

This component is the backend of a prototype aiming at bringing more automation  
to the field of software verification tools targeting rust-based programs.

See [SafePKT description](https://ledgerproject.github.io/home/#/teams/SafePKT)

# Table of contents

 - [Development](#development)
   - [Help](#help)
   - [Install cargo with rustup](#install-cargo-with-rustup)
   - [Configuration](#configuration)
   - [Build the project](#build-the-project)
   - [Build a new release](#build-a-new-release)
   - [Documentation](#documentation)
   - [Run tests](#run-tests)
 - [Deployment](#deployment)
   - [Run the backend](#run-the-backend)
   - [Use nginx as reverse-proxy](#run-nginx-as-reverse-proxy)
 - [Acknowledgment](#acknowledgment)
 - [License](#license)

# Development

## Help

List all the Makefile targets.

```shell
make help
```

## Install cargo with rustup

Download `rustup` and install Rust dependencies [per official instructions](https://rustup.rs/)

```shell
make install-deps
```

## Configuration

Copy the configuration file template and update its entries per your need.

```shell
cp .env{.dist,}
```

- `HOST` - the host where the backend will be available from,
- `PORT` - the port which the backend will be listening on,
- `SOURCE_DIRECTORY` - the directory where the backend will upload source codes to,
- `RVT_DIRECTORY` - the directory where the [rust verifications tools](https://github.com/project-oak/rust-verification-tools) have been cloned,
- `RVT_DOCKER_IMAGE` - the name of a container image pulled from a [registry](https://hub.docker.com/repository/docker/thierrymarianne/contrib-rvt_r2ct-llvm-11) or [built manually](https://project-oak.github.io/rust-verification-tools/about.html),

## Build the project

```shell
make build
```

## Build a new release

Compile a binary and copy it to `./target/release/safepkt-backend`.

```shell
make release
```

## Documentation

Generate the documentation.

```shell
make docs
```

## Run tests

```shell
make test
```

# Deployment

## Run the backend

```shell
./target/release/safepkt-backend
```

## Run nginx as reverse-proxy

Configuration templates for `nginx` are available from [provisioning/web-server/nginx](../../blob/main/provisioning/web-server/nginx).
Configuration files for running the backend with `docker` and `docker-compose` are available from
 - [provisioning/web-server/docker-compose.yml](../../blob/main/provisioning/web-server/docker-compose.yml)
 - [provisioning/web-server/docker-compose.override.yml.dist](../../blob/main/provisioning/web-server/docker-compose.override.yml.dist)  
 allowing to declare 
   - paths to TLS certificates, 
   - docker network
   - basic authentication (as arbitrary source files can be uploaded and compiled)

# Acknowledgment

We're very grateful towards the following organizations, projects and people:
 - the Project Oak maintainers for making [Rust Verifications Tools](https://project-oak.github.io/rust-verification-tools/), a dual-licensed open-source project (MIT / Apache).  
 The RVT tools allowed us to integrate with industrial-grade verification tools in a very effective way. 
 - the KLEE Symbolic Execution Engine maintainers
 - the Rust community at large
 - All members of the NGI-Ledger Consortium for accompanying us  
 [![Blumorpho](../main/img/blumorpho-logo.png?raw=true)](https://www.blumorpho.com/) [![Dyne](../main/img/dyne-logo.png?raw=true)](https://www.dyne.org/ledger/) [![FundingBox](../main/img/funding-box-logo.png?raw=true)](https://fundingbox.com/) [![NGI LEDGER](../main/img/ledger-eu-logo.png?raw=true)](https://ledger-3rd-open-call.fundingbox.com/)

# License

This project is distributed under either the [MIT](../../blob/main/LICENSE-MIT) license or the [Apache](../../blob/main/LICENSE-APACHE) License.
