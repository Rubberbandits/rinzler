#!/bin/bash
# This script is meant to be run on Unix/Linux based systems

echo "*** Initializing Zombienet installation"

# Download zombienet bin
DIST="linux"
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
	ARCH=""
    case $(uname -m) in
		x86_64) ARCH="x64" ;;
		arm)    dpkg --print-architecture | grep -q "arm64" && ARCH="arm64" || ARCH="arm64" ;;
	esac

	DIST+="-${ARCH}"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    DIST="macos"
else
    echo "*** Unknown distribution, stopping..."
	exit
fi

DIR="../zombienet"
mkdir $DIR

FILE_PATH="${DIR}/zombienet"

DOWNLOAD="https://github.com/paritytech/zombienet/releases/download/v1.3.33/zombienet-${DIST}"
wget $DOWNLOAD -O $FILE_PATH
chmod +rx $FILE_PATH

# Install support libraries for compiling substrate binaries
curl https://getsubstrate.io -sSf | bash -s -- --fast

# Download relay chain
mkdir "${DIR}/polkadot"
git clone https://github.com/paritytech/polkadot.git "${DIR}/polkadot"
git checkout release-v0.9.32
cd "${DIR}/polkadot"

# Build relay chain
cargo build --release

cd ../../ # Back to main directory

# Build rinzler
cargo build --release

echo "*** Setup complete, use zombienet.sh in scripts to start a local network."
exit