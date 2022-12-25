#!/usr/bin/env bash

root_path=$(dirname "$(realpath "$0")")

pushd "$root_path" || exit 1
cd frontend || exit 1
npm install
popd || exit 1
