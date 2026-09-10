#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
# Retain Cargo's cache; gui is an explicit feature, not a tray toggle.
cargo build --locked --release --bin nvctl --no-default-features
cargo build --locked --release --bin nvcontrol --no-default-features --features gui
cargo test --locked --lib --no-default-features
