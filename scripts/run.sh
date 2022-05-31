#!/bin/bash
trunk build index.html # --proxy-backend=https://yew.rs/tutorial
cargo run --features=ssr --bin simple_ssr_server -- --dir dist/
