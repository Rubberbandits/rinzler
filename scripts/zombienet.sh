#!/bin/bash
# This script is meant to be run on Unix/Linux based systems

echo "*** Starting zombienet..."

ZOMB_WS_PORT=12345
./zombienet spawn --provider native $(pwd)/rinzler_zombienet_config.toml