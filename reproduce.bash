#!/bin/bash
set -o errexit -o pipefail -o nounset
cd "$(dirname "$0")"

echo 'Recreating working environment...' >&2
rm -rf work
mkdir -p work
cd work

echo 'Creating text files...' >&2
for number in {0..10}; do
  echo "number $number" > "number-$number.txt"
done

echo 'Creating the archive...' >&2
tar cf archive.tar number-*.txt

echo 'Running the Rust program...' >&2
cd ..
cargo run
