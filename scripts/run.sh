#!/bin/bash
trunk build index.html
cargo run --features=ssr --bin simple_ssr_server -- --dir dist/
