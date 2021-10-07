#!/bin/sh

wasm-pack build --release --target no-modules
cp pkg/* docs/pkg
