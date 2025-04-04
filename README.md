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
