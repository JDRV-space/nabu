#!/bin/bash
set -euo pipefail

readonly TRUNK_VERSION="0.21.14"

if [[ "${1:-}" == "--version" ]]; then
    printf '%s\n' "$TRUNK_VERSION"
    exit 0
fi

case "$(uname -s)-$(uname -m)" in
    Darwin-arm64)
        readonly archive_url="https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-aarch64-apple-darwin.tar.gz"
        readonly expected_sha256="764e299dd50d89442a4e96a236349f57961984b701e74d3dbdb39cd1c9f5101e"
        ;;
    Darwin-x86_64)
        readonly archive_url="https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-apple-darwin.tar.gz"
        readonly expected_sha256="f1ba0e3bbe24e0ae219c6d22c33e24e2825c1608dd27c2556e323495110f1a95"
        ;;
    Linux-aarch64)
        readonly archive_url="https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-aarch64-unknown-linux-gnu.tar.gz"
        readonly expected_sha256="b1d8e60e454f7fc182d9a4d95d1506ffbae947d8ba90f8f6f02da93b60f980f9"
        ;;
    Linux-x86_64)
        readonly archive_url="https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz"
        readonly expected_sha256="f2b4680cd239693a646a2795e4633c625328d7b2a044fbe749fa3a2fe9e7036b"
        ;;
    *)
        printf 'Unsupported platform for pinned Trunk release: %s-%s\n' \
            "$(uname -s)" "$(uname -m)" >&2
        exit 1
        ;;
esac

readonly install_directory="${CARGO_HOME:?CARGO_HOME must be set}/bin"
temporary_directory="$(mktemp -d "${TMPDIR:-/tmp}/nabu-trunk.XXXXXX")"
readonly temporary_directory
trap 'rm -rf "$temporary_directory"' EXIT
readonly archive_path="${temporary_directory}/trunk.tar.gz"

curl --proto '=https' --tlsv1.2 -fsSL "$archive_url" -o "$archive_path"

if command -v sha256sum &> /dev/null; then
    actual_sha256="$(sha256sum "$archive_path" | awk '{print $1}')"
else
    actual_sha256="$(shasum -a 256 "$archive_path" | awk '{print $1}')"
fi
readonly actual_sha256

if [[ "$actual_sha256" != "$expected_sha256" ]]; then
    printf 'Trunk archive checksum mismatch: expected %s, got %s\n' \
        "$expected_sha256" "$actual_sha256" >&2
    exit 1
fi

tar -xzf "$archive_path" -C "$temporary_directory" trunk
mkdir -p "$install_directory"
install -m 0755 "${temporary_directory}/trunk" "${install_directory}/trunk"

if [[ "$("${install_directory}/trunk" --version)" != "trunk ${TRUNK_VERSION}" ]]; then
    printf 'Installed Trunk binary reported an unexpected version\n' >&2
    exit 1
fi
