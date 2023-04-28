#!/bin/bash

set -euo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

echo "Building slides"
RUST_LOG=debug mdslides --template ${SCRIPT_DIR}/template.html --output-dir ${SCRIPT_DIR}/slides --mdbook-path ${SCRIPT_DIR} --index-template ${SCRIPT_DIR}/index-template.html
echo "Building book"
RUST_LOG=info mdbook build ${SCRIPT_DIR}
echo "Testing book"
RUST_LOG=info mdbook test ${SCRIPT_DIR}
