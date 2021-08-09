# SafePKT frontend

This project is implemented in the context of the European [NGI LEDGER program](https://ledger-3rd-open-call.fundingbox.com/).

This component is the frontend of a prototype aiming at bringing more automation  
to the field of software verification tools targeting rust-based programs.

See [SafePKT description](https://ledgerproject.github.io/home/#/teams/SafePKT)

## Table of contents

 - [Installaton](#installation)
   - [Configuration](#configuration)
   - [Development](#development)
 - [Deployment](#deployment)
 - [Acknowledgment](#acknowledgment)
 - [License](#license)

## Installation

As you need node.js available,
we recommend the installation of a Node.js version management tool like one of the following:
 - [nvm](https://github.com/nvm-sh/nvm) or
 - [n](https://github.com/tj/n)

Install JavaScript dependencies.

```shell
npm install
```

## Configuration

Copy the `.env.dist` file to `.env` in the project root directory:

```shell
cp .env{.dist,}
```

Update the following entries needed:
 - `API_HOST`, the backend hostname,
 - `API_PORT`, the backend port,
 - `API_SCHEME`, the backend protocol schemeas

## Development

Serve the application with hot reload from `localhost:3000`

```shell
npm run dev
```

# Deployment

Build for production by generating static files

```shell
npm run build
```

Launch server

```shell
npm run start
```

We recommend the use of [Vercel](https://vercel.com)  
to make the deployment process easier in production.

# Acknowledgment

We're very grateful towards the following organizations, projects and people:
 - the JavaScript and NuxtJS community at large
 - All members of the NGI-Ledger Consortium for accompanying us  
  [![Blumorpho](../main/docs/img/blumorpho-logo.png?raw=true)](https://www.blumorpho.com/) [![Dyne](../main/docs/img/dyne-logo.png?raw=true)](https://www.dyne.org/ledger/) [![FundingBox](../main/docs/img/funding-box-logo.png?raw=true)](https://fundingbox.com/) [![NGI LEDGER](../main/docs/img/ledger-eu-logo.png?raw=true)](https://ledger-3rd-open-call.fundingbox.com/)

# License

This project is distributed under either the [MIT](../../blob/main/LICENSE-MIT) license or the [Apache](../../blob/main/LICENSE-APACHE) License.
