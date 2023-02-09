#!/bin/bash
# This script is meant to be run on Unix/Linux based systems

echo "*** Starting zombienet..."

cd "../zombienet"

./zombienet spawn --provider native ../scripts/rinzler-dev-config.toml