# Install

You can install makeup-comparator directly with cargo:
```shell
cargo install makeup-comparator
```
# How to use it

You can get all the information about parameters and script usage using --help:
```shell
makeup-comparator --help
```

# Parameters available

- **-p** | **--product**: The string (name of the product) to search.
- **--max-results**: The maximum number of results to retrieve.
- **--min-similarity**: The real product name compared to the string provided by `--product` minimum similarity needed to pass the threshold
- **--sort-by**: [name, price, similarity, brand, rating] Sorting type
- **--websites**: [sephora-spain, maquillalia, all] Websites to find