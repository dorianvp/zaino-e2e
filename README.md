# Zaino E2E

## Overview

This project contains experimental ent-to-end tests for Zaino.

## Requirements

- [Docker](https://www.docker.com/)
- [Docker Compose](https://github.com/docker/compose)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [Nextest](https://nexte.st)
- A local [Zebrad docker image](https://github.com/ZcashFoundation/zebra/blob/main/docker/Dockerfile)
  - `docker build -t zebrad:test .`
- A local [Zaino docker image](https://github.com/zingolabs/zaino/blob/dev/Dockerfile)
  - `docker build -t zainod:test .`

## Usage

Run the tests using:

```bash
cargo nextest run
```

## Roadmap

- [x] Boot up Zcashd
- [x] Boot up Zebrad
- [ ] Boot up Zaino
- [ ] Connect Zaino to Zebrad
- [ ] Write tests that verify that Zaino provides the same Json-RPC interface as Zcashd
- [ ] Write tests that verify that Zaino provides the same Json-RPC interface as Zebrad

## License

MIT
