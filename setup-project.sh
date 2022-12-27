#!/usr/bin/env bash

root_path=$(dirname "$(realpath "$0")")

pushd "$root_path" || exit 1 > /dev/null
cd frontend || exit 1
npm install
popd || exit 1 > /dev/null

# check if cargo-watch is installed
if ! command -v cargo-watch &> /dev/null
then
    echo "cargo-watch could not be found"
    echo "install cargo-watch? (this is not needed except for comfort) (y/n)"
    read -r answer
    if [ "$answer" != "y" ]; then
        echo "exiting"
        exit 1
    fi
    cargo install cargo-watch
fi