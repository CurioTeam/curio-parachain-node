## Build node from source
Curio executable is available in (releases)[https://github.com/CurioTeam/curio-parachain-node/releases]. 
Latest version you can try to build from source code.

Ensure you have Rust and the support software installed:

```bash
sudo apt update
# May prompt for location information
sudo apt install -y cmake pkg-config libssl-dev git build-essential clang libclang-dev curl libz-dev

curl https://sh.rustup.rs -sSf | sh

rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
rustup update stable
cargo +nightly install --git https://github.com/alexcrichton/wasm-gc
```

You will also need to install the following packages:

Download build and install Curio executable
```bash
cargo install --force --git https://github.com/CurioTeam/curio-parachain-node --tag v2.1.0 node-cli
```

## Upgrade the chain


Download source from git 
```bash
git clone --depth 1 https://github.com/CurioTeam/curio-parachain-node

Make amend runtime and bump up spec_version by 1  in node/runtime/src/lib.rs
```rust
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("curio"),
    impl_name: create_runtime_str!("curio-node"),
    authoring_version: 10,
    // Per convention: if the runtime behavior changes, increment spec_version
    // and set impl_version to 0. If only runtime
    // implementation changes and behavior does not, then leave spec_version as
    // is and increment impl_version.
    spec_version: 1,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
};
```
build new wasm

```bash
cargo build --release
```

take generated new runtime in ./target/release/wbuild/node-runtime/node_runtime.compact.wasm  
and upload it via node web interface using `sudo`
 

[see here](https://substrate.dev/docs/en/tutorials/upgrade-a-chain/)