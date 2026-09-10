#!/usr/bin/env bash
# Local evidence collection only: no package updates, service changes, or GPU writes.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
expected="${1:-auto}"
[[ $# -le 1 ]] || { echo 'Usage: dev/diagnose.sh [auto|arch|fedora|pop]' >&2; exit 2; }
# shellcheck source=/dev/null
source /etc/os-release
arch_family=false
case " $ID ${ID_LIKE:-} " in
    *' arch '*|*' cachyos '*|*' endeavouros '*|*' manjaro '*) arch_family=true ;;
esac
case "$expected" in
    auto) ;;
    arch) "$arch_family" || { echo "Expected Arch family, found $ID" >&2; exit 2; } ;;
    fedora|pop) [[ "$ID" == "$expected" ]] || { echo "Expected $expected, found $ID" >&2; exit 2; } ;;
    *) echo "Unsupported distro: $expected" >&2; exit 2 ;;
esac
failed=0
required() {
    printf '\nChecking:'; printf ' %q' "$@"; printf '\n'
    if ! "$@"; then failed=1; printf 'FAILED: %s\n' "$*" >&2; fi
}
optional() {
    if command -v "$1" >/dev/null 2>&1; then required "$@";
    else printf 'SKIP: %s not installed\n' "$1"; fi
}
printf 'OS: %s\nSession: %s / %s\n' "$PRETTY_NAME" "${XDG_SESSION_TYPE:-unknown}" "${XDG_CURRENT_DESKTOP:-unknown}"
required uname -r
if git rev-parse --show-toplevel >/dev/null 2>&1; then
    required git rev-parse HEAD
    required git status --short
else
    echo 'SKIP: source revision unavailable (exported source tree)'
fi
if [[ -n "${NVCTL:-}" ]]; then
    required sha256sum "$NVCTL"
fi
optional rustc --version
required nvidia-smi --query-gpu=name,pci.bus_id,driver_version,memory.total --format=csv
required modinfo -k "$(uname -r)" -F version nvidia
required modinfo -k "$(uname -r)" -F license nvidia
optional dkms status

inventory_id="$ID"
if "$arch_family"; then inventory_id=arch; fi
# dpkg-query expands the single-quoted package fields.
# shellcheck disable=SC2016
case "$inventory_id" in
    arch) required pacman -Q ;;
    fedora) required rpm -qa --qf '%{NAME} %{VERSION}-%{RELEASE} %{ARCH}\n' ;;
    pop|ubuntu|debian) required dpkg-query -W '-f=${binary:Package} ${Version} ${Architecture}\n' ;;
    *) echo 'SKIP: package inventory for this distro' ;;
esac
required bash dev/test-cli.sh
printf '\nDiagnostics exit status: %s (review capability/doctor findings as well)\n' "$failed"
exit "$failed"
