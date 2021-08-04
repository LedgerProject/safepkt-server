# SafePKT server

A back-end to run static analysis tools against rust-based smart contracts.

## Installation

Copy the configuration file before updating it

```shell
cp env{.dist,}
```

## Help

```shell
make help
```

## Documentation

```shell
make docs
```

## Install cargo with rustup

```shell
make install-deps
```

## Build the project

```shell
make build
```

## Build a new release

```shell
make release
```

## Run the server

```shell
./target/release/safepkt-server
```

## Run tests

```shell
make test
```
