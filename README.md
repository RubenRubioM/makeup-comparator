# makeup-comparator
[![Rust](https://github.com/RubenRubioM/makeup-comparator/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/RubenRubioM/makeup-comparator/actions/workflows/rust.yml)

A CLI project made in Rust to compare different properties for makeup products from different websites.

## Run test locally
```bash
cargo test --workspace -- --include-ignored
```

## Run test coverage locally
```bash
$ CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test --workspace -- --include-ignored
$ grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/
$ $ grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/lcov.info
```

## TODO
- Add a boolean in case the tone in a product exist but it is sold out.
- In SephoraSpain we only get the first 27 items (3 items * 9 rows). The search page has infinite scroll. Find a way to load all the results.