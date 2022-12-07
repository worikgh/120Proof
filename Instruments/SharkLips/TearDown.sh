#!/bin/sh

set -e

pgrep yoshimi && killall yoshimi
pgrep lpx_manager && killall lpx_manager
./lpx_mode 1
./lpx_mode 127
./InitialiseMidi /dev/null

echo SharkLips torn down
