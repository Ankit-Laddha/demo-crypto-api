# Rust BDD API E2E Testing

This project is a Rust-based end-to-end (E2E) testing framework using the Cucumber testing framework for API testing. 
This project includes the setup for Docker to run the tests in a containerized environment.

## Prerequisites

To set up and run this project, you need the following prerequisites installed:

- [Rust](https://www.rust-lang.org/)
- [Docker](https://www.docker.com/products/docker-desktop) (for running tests in Docker)


## Setting Up the Project

### Cloning the Repository

```sh
git clone https://github.com/Ankit-Laddha/demo-crypto-api.git
cd demo-crypto-api
```

### Setting Up Environment Variables
`[IMP]` We are dealing with few secrets for this project. So clone the `.env.template` file available in the root of the project and create another `.env` file. 
Add the actual values for those variables, they are `gitIgnored` to be safe.


## Running Tests

### Without Docker

#### To run all scenarios together
```sh
 cargo test --test cucumber_runner
```

#### To run all specific scenario
```sh
 cargo test --test cucumber_runner -- --tags=@tagname
```
e.g.
```sh
 cargo test --test cucumber_runner -- --tags=@ticker
```
### With Docker

#### Build docker image
```sh
 docker build -t crypto-api .
```
#### To run all scenarios together
```sh
 docker run --rm -it crypto-api
```

#### To run all specific scenario
```sh
 docker run --rm -it crypto-api -- --tags=@tagname
```
e.g.
```sh
 docker run --rm -it crypto-api -- --tags=@ticker
```


