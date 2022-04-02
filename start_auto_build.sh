#!/bin/bash
ls *.toml **/*.rs | entr -s "cargo fmt && wasm-pack build --release --target no-modules && cp pkg/* docs/pkg && rm docs/pkg/*.md"
