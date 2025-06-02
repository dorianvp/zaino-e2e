# Zaino E2E

## Overview

This project contains experimental ent-to-end tests for Zaino.

## Requirements

### Tools

- [Docker](https://www.docker.com/)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [Cargo Make](https://github.com/sagiegurari/cargo-make)
- [Nextest](https://nexte.st)

### Images

Images are automatically downloaded when running `cargo make` or either the `docker_build` or `test` tasks. See the [other tasks](#other-tasks) section for more information.

## Building

This project uses [Cargo Make](https://github.com/sagiegurari/cargo-make) to fetch and build Docker images, and to build and run tests.

## Usage

### Run all tests

Run the tests using:

```bash
cargo make
```

### Other tasks

```bash
# Runs cargo build
cargo make build

# Fetches necessary git repos and builds images
cargo make docker_build

# Runs all of the above and runs tests
cargo make test

# Runs all tests using local images
# (the test framework expects these images to have a particular name)
cargo make test_local
```

## Roadmap

- [x] Boot up Zcashd
- [x] Boot up Zebrad
- [x] Boot up Zaino
- [x] Connect Zaino to Zebrad
- [ ] Write tests that verify that Zaino provides the same Json-RPC interface as Zcashd
- [ ] Write tests that verify that Zaino provides the same Json-RPC interface as Zebrad

## License

MIT
