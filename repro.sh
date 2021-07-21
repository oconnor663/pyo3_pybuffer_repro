#! /usr/bin/env bash

set -e -u -o pipefail

venv_dir="$(mktemp -d)"
virtualenv -p pypy3 "$venv_dir"
source "$venv_dir/bin/activate"
maturin develop

i=1
while python ./test.py ; do
    echo "run number $i"
    i=$(($i + 1))
done
