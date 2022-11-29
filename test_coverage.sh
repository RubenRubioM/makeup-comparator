#!/bin/bash
echo -e "\e[1;32m========== Running test coverage ==========\e[0m"
echo
echo -e "\e[33mCleaning previuos coverages...\e[0m"
cargo clean && mkdir -p coverage/ && mkdir -p scrapped_webs/coverage/ && \
rm -r coverage/* && rm -r scrapped_webs/coverage/* && \
echo -e "\e[32mSuccess: Crate cleaned succesfully\e[0m" 
echo

echo -e "\e[33mCompiling and running tests with code coverage...\e[0m"
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='coverage/cargo-test-%p-%m.profraw' cargo test --workspace -- --include-ignored && \
echo -e "\e[32mSuccess: All tests were executed correctly!\e[0m" 

echo
echo -e "\e[33mGenerating code coverage...\e[0m"
grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/ && \
grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/lcov.info
echo -e "\e[32mSuccess: Code coverage generated correctly!\e[0m" 
