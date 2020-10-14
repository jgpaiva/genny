#!/bin/bash
ls *.toml **/*.rs www/*.html www/*.js | entr -s "cargo fmt && wasm-pack build"
