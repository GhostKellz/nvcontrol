#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
# NVCTL selects the exact installed artifact; otherwise Cargo selects a fresh build.
if [[ -n "${NVCTL:-}" ]]; then
    cli=("$NVCTL")
else
    cli=(cargo run --quiet --locked --bin nvctl --no-default-features --)
fi
failed=0
check() {
    printf '\nChecking:'
    printf ' %q' "$@"
    printf '\n'
    if ! "${cli[@]}" "$@"; then
        failed=1
        printf 'FAILED: %s\n' "$*" >&2
    fi
}
check --version
check --help
check gpu info
check fan info
check power status
check config show
check driver info
check driver capabilities
check display vibrance get
check doctor
exit "$failed"
