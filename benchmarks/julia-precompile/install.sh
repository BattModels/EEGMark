#!/bin/bash

# Fail on error
set -o pipefail
set -e

# Install Juliaup
if ! command -v juliaup &>/dev/null; then
    curl -fsSL https://install.julialang.org | sh -s -- --yes
fi

# Ensure the release channel is install
juliaup add release || true
