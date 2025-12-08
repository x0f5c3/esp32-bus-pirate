#!/bin/bash
# Test script for protocol crate
# The protocol crate needs to be tested on the host, not the ESP32 target

set -e

echo "Testing protocol crate..."
cd protocol

# Temporarily remove cargo config to allow host testing
if [ -f "../.cargo/config.toml" ]; then
    mv ../.cargo/config.toml ../.cargo/config.toml.tmp
fi

# Run tests
cargo test "$@"

# Restore cargo config
if [ -f "../.cargo/config.toml.tmp" ]; then
    mv ../.cargo/config.toml.tmp ../.cargo/config.toml
fi

echo "All protocol tests passed!"
