#!/usr/bin/env bash
set -euo pipefail

function check_deps() {
    deps=("curl" "tar")
    missing=()

    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            missing+=("$dep")
        fi
    done

    if (( ${#missing[@]} > 0 )); then
        echo "Error: missing required dependencies: ${missing[*]}" >&2
        exit 1
    fi
}

function install_containerd() {
    echo "Installing containerd..."
    local VERSION=2.3.5
    local OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
    local ARCH

    case "$(uname -m)" in
        x86_64)  ARCH="amd64" ;;
        aarch64) ARCH="arm64" ;;
        arm64)   ARCH="arm64" ;;
        *)
            echo "Unsupported architecture: $(uname -m)" >&2
            exit 1
            ;;
    esac

    local FILE="containerd-${VERSION}-${OS}-${ARCH}.tar.gz"
    local URL="https://github.com/containerd/containerd/releases/download/v${VERSION}/${FILE}"

    curl -fSL "$URL" -o "/tmp/$FILE"
    tar Cxzvf /usr/local/ /tmp/$FILE

    local SYSDIR="/usr/local/lib/systemd/system"
    local SERVICE="$SYSDIR/containerd.service"

    if ! [ -f $SYSDIR ]; then
        mkdir -p $SYSDIR
    fi

    curl -fSL "https://raw.githubusercontent.com/containerd/containerd/main/containerd.service" -o $SERVICE

    systemctl daemon-reload
    systemctl enable --now containerd
}

function remove_containerd() {
    rm -rf /usr/local/bin/{containerd*,ctr}
    systemctl disable --now containerd
    rm /usr/local/lib/systemd/system/containerd.service
}

function main() {
    check_deps

    if [[ "$EUID" -ne 0 ]]; then
        echo "Error: run as root" >&2
        exit 1
    fi

    install_containerd
}

main
