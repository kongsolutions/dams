# KONGWallet - Decentralized Account Management System (DAMS) Backend (v1-Integration)

## Introduction

Welcome to the primary repository hub for KONGWallet! KONGWallet is a next-generation decentralized crypto wallet built with the clear goal of making digital asset management easy, secure, and accessible for everyone, without sacrificing decentralization or user control.

This project addresses key Web3 pain points such as the complexity of managing multiple wallets/chains, high and unpredictable gas fees, and the barriers to mainstream adoption.

Our core backend (DAMS) is being developed on the **Internet Computer (ICP)** using **Rust**. It employs innovative approaches like **Client-Side Key Derivation (CSKD)** and **Shamir's Secret Sharing (SSS)** for secure account recovery (RAS), while also planning for **Account Abstraction (AA)** integration to optimize fees via the **KONG** token. The v4-Integration architecture focuses on decentralized management of sovereign identities (DID), cryptographic keys, and interaction with blockchain networks.

**Status:** This project is under **active development**. The core codebase resides in **private feature branches** (in our main development repository on Bitbucket/Private GitHub). This public repository serves as the main information hub and future host for public examples/modules.

**Learn More:**
* **[KONGWallet Website/Landing Page](https://kongwallet.io/)**
* **[Follow us in Twitter](https://x.com/kongwallet)**

## Architecture Overview (v1-Integration - 13 Canisters)

The backend architecture follows the v1-Integration model with 13 canisters, optimized for security, scalability, and integration:

1.  **MainIntegrationGateway**: Main entry point for request distribution.
2.  **UserGateway**: Specialized for handling user requests and processes.
3.  **APIGateway**: Specialized for handling requests from external systems/API clients.
4.  **ChallengeGeneration**: Generates cryptographic challenges (nonces).
5.  **SignatureVerification**: Verifies cryptographic signatures with caching.
6.  **RegistryDirectory**: Manages and routes to UserRegistry shards.
7.  **StorageDirectory**: Manages and routes to EncryptedShardStorage shards.
8.  **UserRegistry\_Shard(s)**: Stores user profiles, DID documents, and public keys (sharded).
9.  **EncryptedShardStorage\_Shard(s)**: Securely stores encrypted recovery data (SSS shards) (sharded).
10. **SessionManager**: Manages user sessions and authentication levels.
11. **BlockchainAccessManager**: Manages links to blockchain addresses and network interaction.
12. **SecurityServices**: Consolidated services for audit logs, monitoring, and account recovery (RAS).
13. **CryptoManagerBridge**: Specialized bridge for integration with external APIs like the Crypto Manager API.

## Technology Stack

* **Blockchain:** Internet Computer (ICP)
* **Language (Canisters):** Rust
* **Interfaces:** Candid
* **Storage:** Stable Structures
* **Cryptography:** Ed25519, Argon2id, SHA-256
* **Key Concepts:** Decentralized Identifiers (DID), Client-Side Key Derivation (CSKD), Shamir's Secret Sharing (SSS), Secure Account Recovery (RAS), Multi-Chain Integration, Account Abstraction (AA - planned), Http Outcalls

## Current Status & Progress (as of end Q1 2025 / start Q2 2025)

*Note: This section reflects the status as of Q1/Q2 2025, while the described architecture is the newer v4-Integration model.*

The DAMS backend is under active development on ICP/Rust (in private branches).

* **Milestones Achieved (Q1 2025):** Mobile Web v1 UI, Initial Solana & TON integration (core functions), AI Trending engine (Solana) built, Landing Page launched, Core DAMS implementation started, Initial DEX integration research/work started, Telegram login integration started.
* **Next Steps (Q2 2025):** Finalizing and stabilizing DAMS/RAS implementation (TOP PRIORITY), Launching Fiat On-Ramp, Launching Telegram Mini-App (Beta), Publishing Whitepaper (Draft), **Launching Closed Beta (Target: End of Q2)**.

See our full **Roadmap** for more details.

## Examples (Coming Soon!)

In the future, we plan to add **public code examples** here or in separate repositories within the **kongsolutions** organization. These examples will demonstrate our approaches and skills in using ICP and Rust within the DAMS context.

* [Example 1: Description] - *(Coming Soon)*
* [Example 2: Description] - *(Coming Soon)*

## Team

KONGWallet is being built by a dedicated **full-time team of 6** (1 founder, 5 full-time developers) with expertise in Rust, Frontend (React Native/React/Next.JS), Backend (Nest.JS, Typescript), ML, and secure systems.

* Martin - [Lead DEV]
