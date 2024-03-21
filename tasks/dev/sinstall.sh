#!/bin/bash

./tasks/dev/build.sh

sudo cp ./target/release/pkt /usr/bin/

echo "installed pkt to /usr/bin/pkt"
