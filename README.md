# Substrate Frontier Chain

Frontier is the EVM backbone of Polkadot UI

## Features

Frontier provides a compatibility layer of EVM, so that you can run any Ethereum
dapps on Polkadot, unmodified. Using Frontier, you get access to all of the
Ethereum RPC APIs you are already familiar with, and therefore you can continue
to develop your dapps in your favourite Ethereum developer tools. As a bonus,
you can even run many Ethereum L2s inside Frontier!

Frontier is also a migration framework. Besides the common strategy of direct
state export/import and transaction-level replays, Frontier's Pre-Log Wrapper
Block feature provides a possible method for a zero-downtime live migration.

## Releases

### Primitives

Those are suitable to be included in a runtime. Primitives are structures shared
by higher-level code.

* `fp-consensus`: Consensus layer primitives.
![Crates.io](https://img.shields.io/crates/v/fp-consensus)
* `fp-evm`: EVM primitives. ![Crates.io](https://img.shields.io/crates/v/fp-evm)
* `fp-rpc`: RPC primitives. ![Crates.io](https://img.shields.io/crates/v/fp-rpc)
* `fp-storage`: Well-known storage information.
![Crates.io](https://img.shields.io/crates/v/fp-storage)

### Pallets

Those pallets serve as runtime components for projects using Frontier.
* `pallet-evm`: EVM execution handling. ![Crates.io](https://img.shields.io/crates/v/pallet-evm)
* `pallet-ethereum`: Ethereum block handling. ![Crates.io](https://img.shields.io/crates/v/pallet-ethereum)
* `pallet-dynamic-fee`: Extends the fee handling logic so that it can be changed within the runtime. ![Crates.io](https://img.shields.io/crates/v/pallet-dynamic-fee)

### EVM Pallet precompiles

Those precompiles can be used together with `pallet-evm` for additional
functionalities of the EVM executor.

* `pallet-evm-precompile-simple`: Four basic precompiles in Ethereum EVMs. ![Crates.io](https://img.shields.io/crates/v/pallet-evm-precompile-simple)
* `pallet-evm-precompile-blake2`:  BLAKE2 precompile. ![Crates.io](https://img.shields.io/crates/v/pallet-evm-precompile-blake2)
* `pallet-evm-precompile-bn128`: BN128 precompile. ![Crates.io](https://img.shields.io/crates/v/pallet-evm-precompile-bn128)
* `pallet-evm-precompile-ed25519`: ED25519 precompile. ![Crates.io](https://img.shields.io/crates/v/pallet-evm-precompile-ed25519)
* `pallet-evm-precompile-modexp`: MODEXP precompile.
