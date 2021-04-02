#!/bin/bash

set -e

cargo build --release

cp target/release/hs ~/bin