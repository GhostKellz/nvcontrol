#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
case "${1:-}" in
    "")
        bash dev/test-cli.sh
        ;;
    --vibrance)
        [[ $# == 1 && "${NVCONTROL_RUN_HARDWARE_TESTS:-0}" == 1 ]] || {
            echo 'Set NVCONTROL_RUN_HARDWARE_TESTS=1 to opt into apply/readback/restore.' >&2
            exit 2
        }
        # Test the checkout; this named regression captures and restores exact raw levels.
        cargo test --locked --test regressions live_vibrance_levels_apply_once -- --ignored --exact --nocapture
        ;;
    *) echo 'Usage: dev/test-hardware.sh [--vibrance]' >&2; exit 2 ;;
esac
