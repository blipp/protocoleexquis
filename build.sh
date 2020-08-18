#!/bin/bash
set -e
set -x
sudo docker build -t pe .
sudo docker run --rm --user "$(id -u)":"$(id -g)" -v "/tmp":/usr/src/foo pe cp /usr/src/pe/out/server /usr/src/foo
rsync /tmp/server hs-pe:protocoleexquis
rsync static migrations hs-pe:protocoleexquis
