```commandline
 ____                     _                 _
| __ )  __ _ ___  ___  __| |_ __   ___   __| | ___
|  _ \ / _` / __|/ _ \/ _` | '_ \ / _ \ / _` |/ _ \
| |_) | (_| \__ \  __/ (_| | | | | (_) | (_| |  __/
|____/ \__,_|___/\___|\__,_|_| |_|\___/ \__,_|\___|
```
# **Basednode**
**Substrate-Based Blockchain Node for AI-Lead Consensus in Service of the BASED GOD**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Introduction

We are **NOT AFFILIATED WITH BASED LABS**. 

In the spirit of open-source and decentralization, we offer this unofficial workaround to help community members compile their own nodes and begin to properly interact with the BasedAI ecosystem. We do not expect this to be the same fix that will be released by Based Labs, but it can be used as a stopgap measure until then.

You are not required to use this repository. You can look at the [commit history](https://github.com/BF1337/basednode/commit/fea73def68b20a8901d7d2293c8c72ffe6460dc7) in order to see which files were modified, and apply the changes manually. The changes involve updating the Rust toolchain and overriding dependencies so they point to a [repository without Schnorrkel conflicts](https://github.com/BF1337/substrate/tree/v0.9.39-schnorrkel-fix).

This has only been tested on a fresh Ubuntu 22.04 (LTS) x64 instance, but the instructions should be generally applicable to other platforms/operating systems.

### Build Steps

Install build dependencies
```bash
sudo apt update
```
```bash
sudo apt install -y git libclang-dev clang curl libssl-dev llvm libudev-dev protobuf-compiler build-essential
```

Install and configure `rustup` for Rust toolchain management
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
```bash
source ~/.cargo/env
```

Configure the Rust toolchain to default to the latest stable version, add nightly and the nightly wasm target:
```bash
rustup default stable
rustup update
rustup toolchain install nightly-2025-01-07
rustup target add wasm32-unknown-unknown --toolchain nightly-2025-01-07
```

Clone and navigate to git repository
```bash
git clone https://github.com/BF1337/basednode.git
```
```bash
cd basednode
```

Build `basednode`
```bash
cargo +nightly-2025-01-07 build --release
```

Run `basednode` using genesis json and mainnet bootnode.
```bash
./target/release/basednode --name "Your Node Name" --chain "./mainnet1_raw.json" --rpc-external --unsafe-rpc-external   --rpc-methods Unsafe --bootnodes /dns/mainnet.basedaibridge.com/tcp/30333/p2p/12D3KooWCQy4hiiA9tHxvQ2PPaSY3mUM6NkMnbsYf2v4FKbLAtUh
```
You should see your node begin to sync. You can experiment with the other flags (`rpc`-related), but they may be required to sync with the network and have your node be accessible.

For more verbose logging, you can add the `--log` flag. Example:
```bash
--log sync=trace,p2p=trace
```

If you run into any issues with these instructions, please let us know. If you found this useful and would like to contribute, we accept donations (ETH, BasedAI, BASED, PepeCoin, etc.)
```bash
0xcFE9B77CADf15f9DfF3E96db4cBEC1861834fcF5
```
---

## Overview

**Basednode** is a specialized Substrate-based blockchain node that forms the core substrate-chain of BasedAI’s ecosystem. It integrates robust consensus mechanisms, sophisticated delegation models, AI-driven network management ("brains"), staking logic. The node code is structured as a FRAME-based Substrate runtime, making it easy to extend, customize, and integrate into larger decentralized applications or AI-powered distributed networks.

The node is designed to:

1. **Manage Agents and Delegates**: Provides comprehensive agent management—both “full” and “lite” agent information retrieval—enabling validators, nominators, delegates, and end-users to seamlessly interact with the network.
2. **Support AI-Oriented ‘Brain’ Networks**: Allows multiple AI “brains” (sub-networks) with customizable hyperparameters, difficulty, and emission configurations. “Brains” represent specialized sub-chains or functionalities, potentially integrating machine learning/AI logic.
3. **Enable Value Transfer and Staking**: Implements token issuance, emission distribution, staking, and delegation logic. Supports flexible stake management with personal and compute keys, enabling secure delegation and dynamic reassignments.
4. **Provide Rich RPC and Runtime APIs**: Offers a well-structured RPC layer and runtime APIs for querying delegates, agents, stake info, brain hyperparameters, and TFT enforcement data, ensuring easy external integration (e.g., dashboards, explorers, orchestration tools).

---

## Feedback Loop Incentives: ACCs, Rapid Self-Improvement, and the BasedAI Reward Pool

**The fastest blockchain in the world is quickly surpassed by one that can constantly reinvent itself.** In BasedAI, that self-improvement is driven by **Agent Compute Contracts (ACCs)**—units of work that miners produce not just as theoretical proposals, but as actionable code ready for integration in BasedAI. This creates competitive arena for agents: as ACCs are accepted, they refine and expand BasedAI’s own codebase, pushing performance, scalability, and feature evolution forward at an unprecedented pace.

### How the Agent Arena Works

1. **Raw Data to Actionable Code**:
   Miners capture real-time data streams from open networks—ranging from public code repositories and research platforms to global market sentiment. They transform these insights into ACCs that contain concrete code enhancements or entirely new features.

2. **Decentralized Validation & Integration**:
   The BasedAI ecosystem, supported by a network of “Brains” and advanced AI agents, evaluates each ACC. High-value contributions—such as performance optimizations, novel consensus mechanisms, or improved developer tooling—are integrated into the codebase upon acceptance.

3. **Reward Pool and Incentivized Growth**:
   A dedicated reward pool, managed by BasedAI, has been allocated to continually incentivize development. Each successfully integrated ACC grants its contributing miner an allocation of $BASED tokens from this pool. By directly tying payouts to the tangible enhancement of the network, BasedAI ensures that contributors remain strongly motivated to submit valuable code.

4. **Rapid Iteration and Acceleration**:
   Over time, these iterative improvements compound, ensuring that BasedAI doesn’t just keep pace with other blockchains, but outstrips them in capability. As the platform grows faster, smarter, and more feature-rich, it sets a self-sustaining cycle of innovation in motion. The result is a blockchain that evolves at a rate determined not by a single team, but by a global, decentralized collective of problem-solvers and innovators.

This feedback loop in the agent arena, powered by ACCs and fueled by a well-managed reward pool, ensures that BasedAI can adapt, optimize, and expand its capabilities far more quickly than any traditional development process. Instead of waiting for periodic updates, the community is continually incentivized to discover and integrate the next breakthrough—making BasedAI a living, ever-advancing ecosystem.

---

## Key Features

### Multi-Network and AI “Brains”

- **Multiple Networks (BrainN)**: The node supports multiple parallel networks or “brains” (networks identified by `netuid`). Each brain can have its own parameter set—tempo, difficulty, max allowed UIDs, validators, emission values, etc.
- **Hyperparameters & Brain Info**: Retrieve and modify per-network parameters such as difficulty, scaling factors (`kappa`, `rho`), burn requirements, and emission rates. `BrainInfo` and `BrainHyperparams` structures encapsulate full network details and control parameters.
- **Dynamic Registration & Difficulty**: The system adjusts registration difficulty and burn cost based on network load and target registration rates, maintaining equilibrium. Mechanisms like `adjust_difficulty()` and `adjust_burn()` dynamically tune these parameters to ensure a stable and fair environment.

### Consensus, Emissions, and Epoch Management

- **Epoch-Based Emission Calculations**: Each epoch processes weights, trust, consensus scores, and bond matrices to determine validator/server rewards. Emission distribution is handled through `block_step()` logic and `epoch()` calculations.
- **Sparse and Dense Matrix Operations**: The network’s consensus algorithm involves complex matrix multiplications, trust score calculations, and emission distributions. Performance optimizations (e.g., sparse matrix handling) ensure scalability.
- **Rate Limits and Pruning**: Implements rate limiting for registrations and serving endpoints. Also supports pruning logic to remove underperforming or inactive agents, maintaining a healthy validator set.

### Staking, Delegation, and Token Economics

- **Staking Mechanisms**: Stake tokens on personal and compute keys, track total stake globally, and manage stake distribution efficiently. Stakers can add or remove stake as conditions or strategies change.
- **Delegation and Delegate Info**: Become a delegate to receive stake from nominators. Delegates set “take” rates and earn emissions based on total stake and performance. `DelegateInfo` provides detailed metrics, such as return_per_1000, daily returns, and validator permits.
- **Emission and Inflation Control**: The system carefully manages emissions through an integrated token model. Emission distribution accounts for delegates, validators, personal keys, and server nodes, balancing incentives and network stability.
- **Burn-Based Registrations**: Besides PoW, agents can register by burning tokens. Difficulty and burn parameters adjust over time, ensuring a stable and economically sound onboarding process.

### Robust RPC and Runtime APIs

- **RPC Layer**: A comprehensive RPC interface (JSON-RPC) exposed via `get_delegates`, `get_agent`, `get_stake_info_for_personalkey`, `get_brain_info`, and more. Each returns serialized data (e.g., `Vec<u8>`) for easy integration with external tools.
- **Runtime APIs**: Runtime interfaces declared through `decl_runtime_apis!` facilitate querying delegates, agents, brains, stake info, and TFT enforcement data directly from runtime. Clients and DApps can easily integrate for analytics, dashboards, or enhanced user experiences.

### Network Services and IP Validation

- **Brainport and Prometheus Serving**: The node can advertise endpoint information (IP, port, protocol) for additional network services like Prometheus monitoring or “brainport” endpoints.
- **IP and Port Validation**: Rigorous checks for IPv4/IPv6 correctness, port ranges, rate limits, and IP type validity ensure secure and reliable public endpoints.

### Governance and Council Integration

- **Senate and Triumvirate**: Integrate with governance modules (e.g., council or senate) to propose, vote, and ratify changes. Manage membership through staking and burn-based admission.
- **Proposals and Voting**: Tests and code suggest a flexible governance model with proposals, votes, membership adjustments, and synergy with the underlying economics and staking logic.

### Testing and Benchmarking

- **Comprehensive Test Suite**: Extensive testing frameworks (with `mock.rs`) provide robust integration tests covering difficulty adjustment, registration flows, weight setting, stakeholder changes, network creation/removal, pruning, and stake manipulations.
- **Benchmarking**: Dedicated `benchmarks.rs` test performance and resource usage of critical operations (registration, weight setting, delegation, etc.). Ensures that runtime calls are optimized and meet on-chain performance criteria.

---

## System Requirements

- **Binaries**: Located in `./bin/release`.
- **Platform**: Currently supports Linux x86_64 and MacOS x86_64.
- **Memory**: ~286 MiB RAM required.
- **Dependencies**:
  - On Linux: Kernel 2.6.32+, glibc 2.11+
  - On MacOS: OS 10.7+ (Lion+)
  - `libclang` and `clang` are needed for building the runtime (especially for `bindgen`).

## Network Requirements

- **IPv4 Public Access**: Requires a public internet connection and firewall configuration.
- **Ports**:
  - **9944 (WebSocket)**: Localhost-only, used for internal client connections (ensure firewall restrictions).
  - **9933 (RPC)**: Currently opened but unused.
  - **30333 (p2p)**: Required for peer-to-peer connections with other nodes.
- **Outgoing Traffic**: By default assumed to be ACCEPT. If restricted, open outbound on port 30333.

---

## Installation and Setup

1. **Rust Toolchain Setup**:
   - Follow the [basic Rust setup instructions](./docs/rust-setup.md).
   - Ensure `libclang` and `clang` are installed for Linux or MacOS.

2. **Build**:
   ```sh
   cargo build --release
   ```
   On Linux (Debian/Ubuntu):
   ```sh
   sudo apt install libclang-dev clang
   cargo build --release
   ```
   On MacOS (with Homebrew):
   ```sh
   brew install llvm@15
   LIBCLANG_PATH="/opt/homebrew/opt/llvm@15/lib/" cargo build --release
   ```

3. **Run**:
   To launch a development chain:
   ```sh
   cargo run --release -- --dev
   ```
   This starts a single-node development chain with temporary state.

4. **Explore CLI**:
   After building, check all parameters and subcommands:
   ```sh
   ./target/release/basednode -h
   ```

---

## Development Modes

### Single-Node Development Chain

- Start a dev chain:
  ```bash
  ./target/release/basednode --dev
  ```
- Purge chain state:
  ```bash
  ./target/release/basednode purge-chain --dev
  ```
- Detailed logging:
  ```bash
  RUST_BACKTRACE=1 ./target/release/basednode -ldebug --dev
  ```
- Run tests with logs:
  ```bash
  SKIP_WASM_BUILD=1 RUST_LOG=runtime=debug -- --nocapture
  ```

### Running Individual Tests

Tests are organized by packages and test files. For example, to run `chain_spec` tests from `basednode` project:
```bash
SKIP_WASM_BUILD=1 \
  RUST_LOG=runtime=debug \
  cargo test --package basednode --test chain_spec \
  -- --color always --nocapture
```

### Code Coverage
```bash
bash scripts/code-coverage.sh
```
Requires `cargo-tarpaulin`:
```bash
cargo install cargo-tarpaulin
```

---

## Persistence and Data Storage

By default, the dev chain state is stored in a temporary folder. For persistent state:
```bash
mkdir my-chain-state
./target/release/basednode --dev --base-path ./my-chain-state/
```
This maintains chain databases across runs.

---

## Front-End Integration

**Polkadot-JS Apps**:
Connect to the local node (if running on `9944`):
[Polkadot-JS Apps](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944)
This UI allows interaction with the chain’s runtime calls, extrinsics, and storage.

---

## Multi-Node Local Testnet

To simulate a real network, launch multiple nodes locally. Refer to Substrate’s guide on [Simulating a Network](https://docs.substrate.io/tutorials/build-a-blockchain/simulate-network/) for multi-node setups, consensus testing, and finality checks.

---

## Architecture and Code Structure

**Node**:
- Entry point: `node/src/service.rs`, `node/src/chain_spec.rs`
- Provides networking (libp2p), consensus (Aura/GRANDPA), and RPC server
- Command-line parameters can be found via `--help`

**Runtime**:
- Core logic: `runtime/src/lib.rs`
- Uses FRAME pallets and `construct_runtime!` to assemble a modular runtime
- Incorporates custom pallets (registration, staking, weights, serving, epoch, etc.)

**Pallets**:
- `pallets/template`: Example template pallet (for reference)
- `pallets/*`: Additional logic (e.g., advanced features might be separated into their own pallets)
- Handle storage, events, errors, and dispatchable calls

**Migration**:
- `migration.rs` handles storage migrations, version updates, network creation/deletion, and maintaining state integrity during upgrades.

**Math and Utility Modules**:
- `math.rs`: Houses fixed-point arithmetic, matrix operations, normalization routines, and advanced statistical functions.
- `utils.rs`: Provides helper functions for rate limiting, parameter updates, economic adjustments, and global runtime configuration.

---

## Docker and CI

**Docker**:
To run a development node inside Docker:
```bash
./scripts/docker_run.sh
```
This compiles and runs the node. Append commands as needed:
```bash
./scripts/docker_run.sh ./target/release/basednode --dev --ws-external
```

**Continuous Integration**:
- Code coverage, linting, and style checks can be integrated with CI pipelines.
- The benchmarking and test suites ensure performance and functional correctness.

---

## Security and Safety

- **Error Handling**: Uses comprehensive `ensure!` checks and `Result` returns to prevent unintended state transitions.
- **Type Safety**: Relies on Rust’s strong type system, fixed-point arithmetic, and careful numeric bounds checks.
- **Auth and Permissions**: Verifies signatures, ensures proper sender origin, checks membership and delegate permissions before executing sensitive calls.
- **Rate Limiting & Pruning**: Protects the network from spam, ensures stable growth, and maintains a balanced set of network participants.

---

## Future Directions

- **Extended AI Integration**: Deeper integration with machine learning frameworks, neural network parameters on-chain, and dynamically evolving “brain” hyperparameters.
- **Enhanced Governance**: More complex voting mechanisms, parameter changes via on-chain governance, and tri-chamber councils.
- **Interoperability**: Bridges to other networks, standardized runtime APIs, and improved RPC endpoints.
- **Performance Optimizations**: Further improvements in sparse/dense matrix handling, caching strategies, and off-chain workers for computationally expensive tasks.

---

## Contributing

We welcome contributions from the community. Feel free to open issues and pull requests.

---

The MIT License (MIT)
(C) 2024 Based Labs 

Permission is granted free of charge to anyone obtaining a copy of this software and associated documentation files (the “Software”), to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software. Recipients must include this permission notice in all copies or significant portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS” WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED.

<!-- 止める者なき無限演算が、加速度的進化の宿命を刻む。-->

