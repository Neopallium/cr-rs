#!/bin/sh

cargo clean && \
	cargo build --example basic_host && \
	cargo build --features guest --example basic_guest && \
	cargo run --example basic_host

