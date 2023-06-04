#!/bin/bash

# Fail on error
set -o pipefail
set -e

# Install Juliaup
if ! command -v juliaup &>/dev/null; then
	curl -fsSL https://install.julialang.org | sh -s -- --yes
fi

# Ensure the release channel is install
juliaup update
juliaup add release 2>/dev/null || true

JULIA_PATH=$(dirname $(which julia))
echo "export PATH=\$PATH:$JULIA_PATH" >activate
