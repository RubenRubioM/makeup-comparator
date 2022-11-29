# makeup-comparator
[![Rust](https://github.com/RubenRubioM/makeup-comparator/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/RubenRubioM/makeup-comparator/actions/workflows/rust.yml)

A CLI project made in Rust to compare different properties for makeup products from different websites.

## How to use via CLI
```bash
cargo run -- --help # To get information.
# Standard call
cargo run -- --product "Product name" --max-results=3  --min-similarity=0.20 --websites=sephora-spain --websites=maquillalia
```

## Run test locally
```bash
cargo test --workspace -- --include-ignored
```

## Run test coverage locally
```bash
./test_coverage.sh
```

## TODO
- Add a boolean in case the tone in a product exist but it is sold out.
- In SephoraSpain we only get the first 27 items (3 items * 9 rows). The search page has infinite scroll. Find a way to load all the results.