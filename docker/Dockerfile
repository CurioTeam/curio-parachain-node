FROM ubuntu:20.04

# set chain
ARG CHAIN="mainnet"
ENV CHAIN=$CHAIN

# set node name
ARG NODE_NAME="curio_validator"
ENV NODE_NAME=$NODE_NAME

# set binary URL
ARG BINARY_URL="https://github.com/CurioTeam/curio-parachain-node/releases/download/v2.1.0/curio-2.1.0-beta_amd64.deb"

# install dependencies
RUN apt-get update && apt-get install -y wget

# download and install curio parachain's binary
RUN wget -O curio.deb $BINARY_URL && \
    dpkg --install curio.deb

# copy chain specifications
COPY spec/mainnet.json /etc/curio/mainnet.json
COPY spec/testnet.json /etc/curio/testnet.json

# create database directories
RUN mkdir -p /var/curio/chains/mainnet && \
    mkdir -p /var/curio/chains/testnet

# run node
CMD ["/bin/sh", "-c", "/usr/local/bin/curio \
        --chain /etc/curio/${CHAIN}.json \
        --base-path /var/curio \
        --name $NODE_NAME \
        --node-key-file /etc/curio/node.key \
        --rpc-cors all \
        --rpc-methods unsafe \
        --validator \
        --rpc-external \
        --ws-external  \
        --ws-port 9944  \
        --rpc-port 9933 \
    "]