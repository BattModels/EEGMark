#!/bin/bash

# Fail on error
set -o pipefail
set -e

# Ensure the release channel is installed
juliaup update
juliaup add release 2>/dev/null || true
