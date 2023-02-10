# Substrate Cumulus Parachain Template

A new [Cumulus](https://github.com/paritytech/cumulus/)-based Substrate node, ready for hacking ☁️..

This project is originally a fork of the
[Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template)
modified to include dependencies required for registering this node as a **parathread** or
**parachain** to a **relay chain**.

The stand-alone version of this template is hosted on the
[Substrate Devhub Parachain Template](https://github.com/substrate-developer-hub/substrate-parachain-template/)
for each release of Polkadot. It is generated directly to the upstream
[Parachain Template in Cumulus](https://github.com/paritytech/cumulus/tree/master/parachain-template)
at each release branch using the
[Substrate Template Generator](https://github.com/paritytech/substrate-template-generator/).

👉 Learn more about parachains [here](https://wiki.polkadot.network/docs/learn-parachains), and
parathreads [here](https://wiki.polkadot.network/docs/learn-parathreads).


🧙 Learn about how to use this template and run your own parachain testnet for it in the
[Devhub Cumulus Tutorial](https://docs.substrate.io/tutorials/v3/cumulus/start-relay/).


## Running benchmarks with build 
cargo build --features runtime-benchmarks ; ./target/debug/paratensor benchmark pallet --pallet pallet_paratensor --extrinsic "*" --output ~/Desktop/out.txt --execution wasm

## Running tests with debug
SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test your_test_name -- --nocapture

## Running a local ZombieNet.
Step 1: move tar into opentensor/rinzler (get the zombie net tar from someone.)
Step 2: cd into opentensor/rinzler ( cd into this repo. )
Step 3: tar -xzvf scripts.tar.gz scripts ( untar the script here.)
Step 4: sudo chmod a+x ./scripts/zombienet/OSX/zombienet (Set the permission on the binery, may need to switch from OSX to linux.)
Step 5: ./scripts/zombienet/OSX/zombienet spawn -p native ./scripts/zombienet/zombienet.toml (run the script in that directory like this)
