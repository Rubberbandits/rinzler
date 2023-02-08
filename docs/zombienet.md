# Zombienet

## Summary

Zombienet allows quick creation of test networks in local environments for the Polkadot ecosystem. This makes end-to-end testing much more efficient as going from build to execution becomes a single command.

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
