#!/bin/sh

wasm-pack build --release --target no-modules
cp pkg/* docs/pkg
rm docs/pkg/*.md
