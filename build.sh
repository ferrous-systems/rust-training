#!/bin/bash

#
# Ferrous Systems Cloudflare Deployment Script
#
# Copyright (c) Ferrous Systems, 2026
#
# This script will find every tag in this repo and build the material.

set -euo pipefail

# We only support macOS (the x86 binaries work OK on Apple Silicon), or x86-64 Linux
if [ "$(uname)" == "Darwin" ]; then
    ./mdbook --version || curl -sSL https://github.com/rust-lang/mdBook/releases/download/v0.4.40/mdbook-v0.4.40-x86_64-apple-darwin.tar.gz | tar -xvzf -
    dot -V || brew install graphviz
    mdbook-graphviz --version || cargo install mdbook-graphviz --locked
    ./mdslides --version || ( curl -sSL https://github.com/ferrous-systems/mdslides/releases/download/v0.7.2/mdslides-x86_64-apple-darwin.tar.xz | tar -xvJf - \
        && mv ./mdslides-*/mdslides . \
        && rm -rf ./mdslides-*/ )
else
    ./mdbook --version || curl -sSL https://github.com/rust-lang/mdBook/releases/download/v0.4.40/mdbook-v0.4.40-x86_64-unknown-linux-gnu.tar.gz | tar -xvzf -
    dot -V || ( curl -ssL https://github.com/restruct/dot-static/raw/refs/heads/master/x64/dot_static -o ./dot && chmod a+x ./dot )
    ./mdbook-graphviz --version || ( curl -sSL https://github.com/dylanowen/mdbook-graphviz/releases/download/v0.2.1/mdbook-graphviz_v0.2.1_x86_64-unknown-linux-musl.zip -o mdbook-graphviz.zip \
        && unzip mdbook-graphviz.zip \
        && rm mdbook-graphviz.zip \
        && chmod a+x ./mdbook-graphviz )
    ./mdslides --version || ( curl -sSL https://github.com/ferrous-systems/mdslides/releases/download/v0.7.2/mdslides-x86_64-unknown-linux-gnu.tar.xz | tar -xvJf - \
        && mv ./mdslides-*/mdslides . \
        && rm -rf ./mdslides-*/ )
fi

# Must be an absolute path, otherwise mdbook puts the output in the wrong place
OUTPUT_DIR=$(pwd)/html
VERSION_FILE="${OUTPUT_DIR}/history/index.html"

# Clean out whatever exists, and make the index (before we do any git checkouts
# and lose the files from this checkout)
rm -rf "${OUTPUT_DIR}"
mkdir -p "${OUTPUT_DIR}"
mkdir -p "${OUTPUT_DIR}/history"
cp ./_redirects "${OUTPUT_DIR}/_redirects"
cp ./index-top.html "${VERSION_FILE}"

# Build the book and slides
function build_and_store {
    mkdir -p "${OUTPUT_DIR}/$1"
    # Build the book first, because mdbook will create any empty sections
    # The PATH override lets it find our local copy of mdbook-graphviz or mdbook-mermaid
    PATH=$PATH:. ./mdbook build -d "${OUTPUT_DIR}/$1/book" ./training-slides
    # Then build the slides
    PATH=$PATH:. RUST_LOG=info ./mdslides --template ./training-slides/template.html \
        --output-dir "${OUTPUT_DIR}/$1/slides" \
        --mdbook-path ./training-slides \
        --index-template ./training-slides/index-template.html
    cp -r "${OUTPUT_DIR}/$1/book/images" "${OUTPUT_DIR}/$1/slides"
}

# Build what we currently have checked out
build_and_store latest

# Fetch all the git tags (in case this is some kind of shallow clone)
git fetch --tags


for tag in $(git tag | sort -V); do
    url1="https://github.com/ferrous-systems/rust-training/releases/download/${tag}/rust-training-${tag}.zip"
    url2="https://github.com/ferrous-systems/rust-training/releases/download/${tag}/output.zip"
    echo "Unpacking ${tag}..."
    if [ ! -f "${tag}.zip" ]; then
        curl -sfSL "${url1}" -o "${tag}.zip" || curl -sfSL "${url2}" -o "${tag}.zip"
    fi
    # Make a place to put the rendered output
    mkdir -p "${OUTPUT_DIR}/${tag}"
    unzip -q "${tag}.zip" -d "${OUTPUT_DIR}/${tag}"
    mv "${OUTPUT_DIR}/${tag}"/*/*-book "${OUTPUT_DIR}/${tag}/book"
    mv "${OUTPUT_DIR}/${tag}"/*/*-presentation "${OUTPUT_DIR}/${tag}/slides"
    rm -rf "${OUTPUT_DIR}/${tag}"/rust-*
    rm -rf "${OUTPUT_DIR}/${tag}"/output
    echo "<li>${tag} (<a href=\"/${tag}/slides\">Slides</a> | <a href=\"/${tag}/book\">Book</a>)</li>" >> "${VERSION_FILE}"
done

cat ./index-bottom.html >> "${VERSION_FILE}"
