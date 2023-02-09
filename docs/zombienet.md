# Zombienet

## Summary

Zombienet allows quick creation of test networks in local environments for the Polkadot ecosystem. This makes end-to-end testing much more efficient as going from build to execution becomes a single command.

## Installation

### Downloading binaries

You can easily download platform-specific binaries from `github.com/paritytech/zombienet/releases/latest`

Example for Linux-x64:
`wget https://github.com/paritytech/zombienet/releases/download/v1.3.33/zombienet-linux-x64`

Distributions

```bash
linux-arm64
linux-x64
macos
```

### From source

#### Requirements

- [Node.js](https://nodejs.org/) if you are not using the self contained linux or macos
  [releases](https://github.com/paritytech/zombienet/releases).
- [Kubernetes](https://kubernetes.io) cluster to use `kubernetes` target (`kubectl` command installed).
- [Podman](https://podman.io) to use `podman` target.

You need first to *clone* this repository and run:

```bash
cd zombienet/javascript
npm install
npm run build
```

#### Download and install needed artifacts (optional)

For an easier and faster setup of your local environment, run:

```bash
node dist/cli.js setup <binaries>
```

This allows to use the `setup` script, making everything ready for a ZombieNet dev environment.

You can use the following arguments:

`--help` shows the different options and commands for using the Zombienet CLI.
`--binaries` or `-b`: enables providing the binaries that you want to be downloaded and installed during the setup. Possible options: `polkadot`, `polkadot-parachain`.

For example:

```bash
node dist/cli.js setup polkadot polkadot-parachain
```

> Note: If you are using macOS please clone the [Polkadot repo](https://github.com/paritytech/polkadot) and run it locally. At the moment there is no `polkadot` binary for MacOs.

The command above will retrieve the binaries provided and try to download and prepare those binaries for usage.
At the end of the download, the `setup` script will provide a command to run in your local environment in order to add the directory where the binaries were downloaded in your $PATH var, for example:

```bash
Please add the dir to your $PATH by running the command: export PATH=/home/<user>/zombienet/dist:$PATH
```

#### Build adder-collator (needed for running examples with native provider)

You can build it from source like this

```bash
git clone git@github.com:paritytech/polkadot
cd polkadot
cargo build --profile testnet -p test-parachain-adder-collator
export PATH=$(pwd)/target/testnet:$PATH
```

#### Using Zombienet

With the above steps completed, the `zombienet` CLI is ready to run. Instead of using the self-contained binary command `./zombienet-{{DISTRO}}` you run `node dist/cli.js`

## CLI Usage

For this example we will use the `macos` version of the executable.

```bash
./zombienet-macos
Usage: zombienet [options] [command]

Options:
  -c, --spawn-concurrency <concurrency>  Number of concurrent spawning process to launch, default is 1
  -p, --provider <provider>              Override provider to use (choices: "podman", "kubernetes", "native")
                                         default: kubernetes
  -d, --dir <path>                       Directory path for placing the network files instead of random temp one (e.g. -d /home/user/my-zombienet)
  -f, --force                            Force override all prompt commands
  -m, --monitor                          Start as monitor, do not auto cleanup network
  -h, --help                             display help for command

Commands:
  spawn <networkConfig> [creds]  Spawn the network defined in the config
  test <testFile>                Run tests on the network defined
  version                        Prints zombienet version
  help [command]                 display help for command
```

## Startup

Zombienet has three modes of operation, which provides a variety of environments for runtimes. These three are Kubernetes, podman and native. Podman can execute docker images directly from image names in the config, while native runs binaries specified by path.

An example startup command:
`./zombienet-linux-x64 --provider native spawn scripts/rinzler_zombienet_config.toml`

## Configuration

When starting up zombienet, you pass the path to your startup config. This defines what image/binary to run for the relay chain, validator configuration, and parachain collators to configure at genesis.

Basic options in config.toml to know:

```toml
default_command: bin path/image for relay chain
chain: chain type passed to relaychain/parachain build-spec command

Add relaychain validators with [[relaychain.nodes]]

Defining [[parachains]] allows you to specify any parachains you want to register at genesis. 
Takes id (int), cumulus_based (bool), chain (string)

Define [parachains.collator] as a member of [[parachains]] in order to specify the parachain name and bin/image to run as a collator.
```

You can also construct HRMP channels at genesis using [[hrmp_channel]] like so:

```toml
[[hrmp_channel]]
sender = 1000
recipient = 1001
maxCapacity = 8
maxMessageSize = 8192

[[hrmp_channel]]
sender = 1001
recipient = 1000
maxCapacity = 8
maxMessageSize = 8192
```

## Polkadot.js Frontend

Using the chain frontend hosted at polkadot.js.org, you can connect to your local zombienet. Ports change every time you launch zombienet, so you'll have to use the port that the collator section of zombienet's printout. Zombienet provides a handy direct link in the node information output when all validators/collators have finished booting. Example:

`https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:34083#/explorer`
