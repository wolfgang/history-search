#!/bin/bash

set -e

cargo build --release

cp target/release/rp ~/bin