#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Build the book first, because mdbook will create any empty sections
echo "Building book"
RUST_LOG=info mdbook build ${SCRIPT_DIR}
# Then build the slides
echo "Building slides"
RUST_LOG=debug mdslides --template ${SCRIPT_DIR}/template.html --output-dir ${SCRIPT_DIR}/slides --mdbook-path ${SCRIPT_DIR} --index-template ${SCRIPT_DIR}/index-template.html
# TODO: move assets copying to mdslides
cp -r "${SCRIPT_DIR}/book/images" "${SCRIPT_DIR}/slides"
# Then run the tests (which is slow)
echo "Testing book"
RUST_LOG=info mdbook test ${SCRIPT_DIR}
