#!/bin/bash
set -eu

cd harptabber-gui/
trunk build --release
sed -i -e "s@'/@'./@g" -e 's@"/@"./@g' dist/index.html
