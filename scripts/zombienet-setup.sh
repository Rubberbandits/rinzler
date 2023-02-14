#!/bin/bash

echo "*** Initializing Zombienet installation"

WORKING_DIR=$(pwd)
BRANCH="polkadot-v0.9.32"

# Download zombienet bin
DIST="linux"
CHECKSUM=""
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
	ARCH=""
	case $(uname -m) in
		x86_64) ARCH="x64"; CHECKSUM="d864e830850bdd10718fd3ec27aa6bdd" ;;
		arm)    dpkg --print-architecture | grep -q "arm64" && ARCH="arm64" || ARCH="arm64"; CHECKSUM="c328b2eced5b21b313dc9af7558f5557" ;;
	esac

	DIST+="-${ARCH}"
elif [[ "$OSTYPE" == "darwin"* ]]; then
	DIST="macos"
	CHECKSUM="c4511801b40305c13e45edbd506961be"
else
	echo "*** Unknown distribution, stopping..."
	exit
fi

DIR="${WORKING_DIR}/zombienet"
mkdir $DIR

FILE_PATH="${DIR}/zombienet"

DOWNLOAD="https://github.com/paritytech/zombienet/releases/download/v1.3.33/zombienet-${DIST}"
echo "*** Downloading zombienet..."
wget -q $DOWNLOAD -O $FILE_PATH
if [[ "$(md5sum ${FILE_PATH} | awk '{print $1}')" != $CHECKSUM ]]; then
	echo "Zombienet binary checksum not valid, exiting."
	exit
fi
chmod +rx $FILE_PATH

echo "*** Installing substrate support libraries"

# Install support libraries for compiling substrate binaries
# verify md5
curl https://getsubstrate.io -sSf > support_install.sh
if [[ "$(md5sum support_install.sh | awk '{print $1}')" != "7296b9d45a89e973528c3ae31719ff08" ]]; then
	echo "Substrate library script checksum not valid, exiting."
	exit
fi
chmod +rx support_install.sh
bash support_install.sh
rm support_install.sh

echo "*** Downloading relay chain"

# Download relay chain
mkdir "${DIR}/polkadot"
git clone https://github.com/paritytech/polkadot.git "${DIR}/polkadot"
git fetch --all
git checkout $BRANCH
cd "${DIR}/polkadot"

# Build relay chain
cargo build --release

cd $WORKING_DIR # Back to working directory

# Build rinzler
cargo build --release

echo "*** Setup complete, use zombienet.sh in scripts to start a local network."
exit