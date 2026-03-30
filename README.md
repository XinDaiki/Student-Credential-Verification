# Allowance Tracker

## Overview
This is a decentralized allowance tracker built on the Stellar network using the Soroban SDK and Rust. Inspired by the need to efficiently manage academic stipends and science and technology scholarship grants, this contract is designed to securely hold funds and distribute them based on completed milestones. 

This repository serves as my first functional smart contract and was built in preparation for the Stellar Smart Contract Bootcamp.

## Contract Details
- **Network:** Stellar Testnet
- **Contract ID:** `CAWMA6APYPS2DRNGQXX7UJA2G6Z7WM3QI3GQ25FVYY53AZGOB6KOTUM3`
- **Stellar Expert Explorer:** [View on Testnet Explorer](https://stellar.expert/explorer/testnet/contract/CAWMA6APYPS2DRNGQXX7UJA2G6Z7WM3QI3GQ25FVYY53AZGOB6KOTUM3)

## Setup and Installation

### Prerequisites
Ensure you have the following installed:
- Rust (v1.84.0 or newer)
- Stellar CLI (v25.2.0)
- WebAssembly target (`wasm32-unknown-unknown`)

### Building the Contract
1. Clone this repository to your local machine.
2. Navigate to the project directory.
3. Build the contract by running:
   ```bash
   cargo build --target wasm32-unknown-unknown --release