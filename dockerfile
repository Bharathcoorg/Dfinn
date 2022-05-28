FROM ubuntu as builder
RUN apt-get update
RUN apt-get install -y git && apt-get install -y curl
RUN git clone https://github.com/bharathcoorg/Dfinn -b main-net-runtime
RUN cd Dfinn && \
    git checkout $(git describe --tags --abbrev=0) && \
    apt-get install -y build-essential && \
    apt-get install -y clang && \
    apt-get install -y jq && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
        export PATH="$PATH:$HOME/.cargo/bin" && \
        rustup toolchain install nightly-2021-11-11 && \
        rustup target add wasm32-unknown-unknown --toolchain nightly-2021-11-11 && \
        cargo +nightly-2021-11-11 build --release

# /\-Build Stage | Final Stage-\/

FROM docker.io/library/ubuntu:20.04
COPY --from=builder /Dfinn/target/release/dfinn-node /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /dfinn-node dfinn-node && \
        mkdir -p /dfinn-node/.local/share && \
        mkdir /data && \
        chown -R dfinn-node:dfinn-node /data && \
        ln -s /data /dfinn-node/.local/share/dfinn-node && \
        rm -rf /usr/bin /usr/sbin

COPY --from=builder /Dfinn/extras/customSpecRaw.json /data

USER dfinn-node
EXPOSE 30333 9933 9944
VOLUME ["/data"]

EXPOSE 30333 9933 9944

ENTRYPOINT ["/usr/local/bin/dfinn-node"]

# You should be able to run a validator using this docker image in a bash environmment with the following command:
# docker run <docker_image_name> --chain /data/customSpecRaw.json $(curl -s https://raw.githubusercontent.com/bharathcoorg/Dfinn/main/docs/run-a-validator.md | grep -o -m 1 -E "\-\-bootnodes \S*") --validator --name "Validator-Name"
