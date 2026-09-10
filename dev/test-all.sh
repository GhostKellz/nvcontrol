#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
cargo fmt --all --check
bash dev/test-features.sh
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-features
printf '%s\n' 'Build and automated tests passed. Live diagnostics and GUI/TUI acceptance are separate; see docs/testing/ci-workflow.md.'
