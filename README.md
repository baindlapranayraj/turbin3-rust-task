# Turbin3 Rust Solana CLI Toolkit

![Turbin3 Logo](https://media.licdn.com/dms/image/v2/D560BAQFCvzGKgNWmTQ/company-logo_200_200/company-logo_200_200/0/1733371087700/turbin3_logo?e=2147483647&v=beta&t=zIKTy9Hx-t4D9lEc-UjcEFA45b6-rzgk_sICHeTR46c)

This project is a **Turbin3** assignment designed to create a Command Line Interface (CLI) for interacting with the Solana blockchain. The toolkit, developed in Rust, facilitates keypair management, base58 encoding/decoding, SOL airdrops, and program interactions. This assignment demonstrates blockchain proficiency and Rust programming capabilities.

---

## Features

### **1. Keypair Management**
- **Generate Keypair**: Generate new Solana keypairs securely using `Keypair::new()`.
- **Retrieve Keypair**: Load existing keypairs from JSON data stored in environment variables (`KEY_PAIR` or `WALLET_KEY`).

### **2. Base58 Encoding/Decoding**
- **base58_to_wallet**: Convert base58-encoded public keys into wallet byte arrays.
- **wallet_to_base58**: Encode wallet byte arrays into base58 format for readability and sharing.

### **3. Airdrop SOL**
Request airdrops of 2 SOL (default) on Solana's Devnet to a specified public key, simplifying testing and development.

### **4. Transfer SOL**
Facilitate SOL transfers between accounts with support for sending all available SOL minus transaction fees.

### **5. Program Interaction**
- **Turbin3PrereqProgram**: Interact with a custom Solana program to demonstrate Solana program architecture.
- **Execute Transactions**: Use program arguments to complete specific tasks on-chain, highlighting programmatic blockchain interaction.

### **6. Interactive CLI**
The CLI provides an easy-to-use interface for:
- Converting between base58 and wallet byte arrays.
- Initiating SOL transactions and program interactions.

---

## Transaction Proof
https://explorer.solana.com/tx/2pEFfRKbMW4fMzUNXqm3cmuD8fNfwPAoP5XyUhwbLCpwMgY9ne1RpkYAy55Ni52d7eAMLetKRtKe7NTSHHkNzgx4?cluster=devnet
