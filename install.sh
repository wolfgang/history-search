#!/bin/sh

set -e

Dest=$1

[ -z $Dest ] && Dest="$HOME/bin"
if [ ! -d $Dest ]; then
  echo Directory $Dest does not exist, attempting to create it
  mkdir -p $Dest
fi

echo Installing to: $Dest/hs
cargo build --release
cp target/release/hs $Dest