#!/bin/bash

set -e

THISDIR=$(dirname $0)
cd $THISDIR

export SWIFT_BRIDGE_OUT_DIR="$(pwd)/generated"
# Build the project for the desired platforms:
cargo build --features "swift" --target x86_64-apple-darwin
mkdir -p ./target/universal-macos/debug

lipo \
    ./target/x86_64-apple-darwin/debug/librlansync_lib.a -create -output \
    ./target/universal-macos/debug/librlansync_lib.a

swift-bridge-cli create-package \
  --bridges-dir ./generated \
  --out-dir rlansync_lib \
  --macos target/universal-macos/debug/librlansync_lib.a \
  --name rlansync_lib