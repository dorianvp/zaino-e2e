# Zaino E2E

## Overview

This project contains experimental ent-to-end tests for Zaino.

## Requirements

- [Docker](https://www.docker.com/)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [Nextest](https://nexte.st)
- A Zebrad image
- A Zaino image

## Usage

Run the tests using:

```bash
cargo nextest run
```

## Roadmap

- [] Boot up Zcashd
- [] Boot up Zaino
- [] Boot up Zebrad
- [] Connect Zaino to Zcashd
- [] Connect Zaino to Zebrad
- [] Write tests that verify that Zaino provides the same Json-RPC interface as Zcashd
- [] Write tests that verify that Zaino provides the same Json-RPC interface as Zebrad

## License

MIT
