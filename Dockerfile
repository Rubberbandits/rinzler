FROM ubuntu:22.10 AS base
SHELL ["/bin/bash", "-c"]

# This is being set so that no interactive components are allowed when updating.
ARG DEBIAN_FRONTEND=noninteractive

LABEL ai.opentensor.image.authors="operations@opentensor.ai" \
        ai.opentensor.image.vendor="Opentensor Foundation" \
        ai.opentensor.image.title="opentensor/paratensor" \
        ai.opentensor.image.description="Opentensor Paratensor Blockchain"

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && \
        DEBIAN_FRONTEND=noninteractive apt-get install -y \
                build-essential \
                git make clang curl \
                libssl-dev llvm libudev-dev protobuf-compiler \
                curl && \
# apt cleanup
        apt-get autoremove -y && \
        apt-get clean && \
        find /var/lib/apt/lists/ -type f -not -name lock -delete;

# Install cargo and Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN apt remove -y curl

RUN rustup default stable
RUN rustup update

RUN rustup update nightly
RUN rustup target add wasm32-unknown-unknown --toolchain nightly



FROM base as collator

RUN mkdir /root/paratensor
COPY ./ /root/paratensor/
RUN cd /root/paratensor && cargo build --release

CMD ["/root/paratensor/target/release/paratensor --chain latest-raw.json --base-path /tmp/parachain/paratensor --port 40334 --rpc-cors all --rpc-port 9945 --ws-port 8845 --bootnodes /ip4/164.92.84.57/tcp/40334/p2p/12D3KooWRGV7Aem9hJYXpBWBrZrFNnyrD3GxuhsVto2aAdiqYHys -- --execution wasm --chain chain-spec-4val--22Sep-raw.json  --port 30344 --ws-port 9978 --rpc-cors all --bootnodes /ip4/137.184.147.81/tcp/30333/p2p/12D3KooWJPcxdiKNq2HDJ7kZN8XTaTmx6c87YcKnWywrfSSE4Tui --sync warp"]
