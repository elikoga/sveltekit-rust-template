#!/usr/bin/env bash

OPTIND=1

set -eo pipefail

# This script renames the project in the current directory.

# Usage: rename-project.sh [-v|-d|--verbose|--debug] <new-project-name>

root_path=$(dirname "$(realpath "$0")")
debug=0
new_project_name=""

while getopts "vhd-:" opt; do
    case "$opt" in
    h)
        echo "Usage: rename-project.sh [-v|-d|--verbose|--debug] <new-project-name>"
        exit 0
        ;;
    v)  debug=1
        ;;
    d)  debug=1
        ;;
    -)  case "${OPTARG}" in
            verbose) debug=1 ;;
            debug) debug=1 ;;
            *) echo "Invalid option: --${OPTARG}" >&2
               exit 1 ;;
        esac ;;
    *)  echo "Invalid option: -$OPTARG" >&2
        exit 1 ;;
    esac
done

shift $((OPTIND-1))

[ "$1" = "--" ] && shift

if [ $# -eq 0 ]; then
    echo "No new project name specified."
    exit 1
fi

new_project_name=$1

if [ $debug -eq 1 ]; then
    set -x
    echo "root_path: $root_path"
    echo "new_project_name: $new_project_name"
fi
set -u

# Get old project name from ./current_project_name.txt
old_project_name=$(cat "$root_path/current_project_name.txt")

if [ $debug -eq 1 ]; then
    echo "old_project_name: $old_project_name"
fi

# Remove unneeded files
pushd "$root_path"
cd "$root_path/backend"
cargo clean
echo "Removed backend/target"
popd

# Rename project in Cargo.toml
sed -i "s/$old_project_name/$new_project_name/g" "$root_path/backend/Cargo.toml"

# Rename project in ./frontend/package.json
sed -i "s/$old_project_name/$new_project_name/g" "$root_path/frontend/package.json"

# List all files containing the old project name
files_to_rename=$(grep -ril "$old_project_name" "$root_path" || true)

# Rename project in current_project_name.txt
printf "%s" "$new_project_name" > "$root_path/current_project_name.txt"

# Print summary
echo "Renamed project from $old_project_name to $new_project_name in:"
echo "backend/Cargo.toml"
echo "frontend/package.json"
echo "You may need to rename the project in other files:"
echo "$files_to_rename"
