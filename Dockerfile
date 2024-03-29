# **NOTE**: This docker file expects to be run in a directory outside of subtensor.
# It also expects two build arguments, the bittensor snapshot directory, and the bittensor
# snapshot file name.

# This runs typically via the following command:
# $ docker build -t subtensor . --platform linux/x86_64 --build-arg SNAPSHOT_DIR="DIR_NAME" --build-arg SNAPSHOT_FILE="FILENAME.TAR.GZ"  -f subtensor/Dockerfile


FROM ubuntu:22.10
SHELL ["/bin/bash", "-c"]

# metadata
ARG VCS_REF
ARG BUILD_DATE
ARG SNAPSHOT_DIR
ARG SNAPSHOT_FILE

# This is being set so that no interactive components are allowed when updating.
ARG DEBIAN_FRONTEND=noninteractive

LABEL ai.opentensor.image.authors="operations@opentensor.ai" \
        ai.opentensor.image.vendor="Opentensor Foundation" \
        ai.opentensor.image.title="opentensor/rococo_paratensor" \
        ai.opentensor.image.description="Opentensor Paratensor Blockchain" \
        ai.opentensor.image.revision="${VCS_REF}" \
        ai.opentensor.image.created="${BUILD_DATE}" \
        ai.opentensor.image.documentation="https://docs.bittensor.com"

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && \
        DEBIAN_FRONTEND=noninteractive apt-get install -y \
                ca-certificates \
                curl \
		clang && \
# apt cleanup
        apt-get autoremove -y && \
        apt-get clean && \
        find /var/lib/apt/lists/ -type f -not -name lock -delete;



# Install cargo and Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN mkdir -p paratensor/scripts
RUN mkdir -p paratensor/specs

COPY scripts/init.sh paratensor/scripts/init.sh
COPY specs/* paratensor/specs/

RUN chmod 744 paratensor/scripts/init.sh
RUN paratensor/scripts/init.sh

COPY ./target/release/paratensor /usr/local/bin

RUN /usr/local/bin/paratensor --version

RUN apt remove -y curl

EXPOSE 30333 30334 9944 9945 9946