# KONGWallet - Decentralized Account Management System (DAMS) Backend

## Introduction

Welcome to the primary repository hub for KONGWallet! KONGWallet is a next-generation decentralized crypto wallet built with the clear goal of making digital asset management **easy, secure, and accessible for everyone**, without sacrificing decentralization or user control.

This project addresses key Web3 pain points such as the complexity of managing multiple wallets/chains, high and unpredictable gas fees, and the barriers to mainstream adoption.

Our core backend (DAMS) is being developed on the **Internet Computer (ICP)** using **Rust**. It employs innovative approaches like **Client-Side Key Derivation (CSKD)** and **Shamir's Secret Sharing (SSS)** for secure account recovery (RAS), while also planning for **Account Abstraction (AA)** integration to optimize fees via the **KONG token**.

**Status:** This project is under **active development**. The core codebase resides in **private feature branches** (in our main development repository on Bitbucket/Private GitHub). This public repository serves as the main information hub and future host for public examples/modules.

**Learn More:**
* **[KONGWallet Website/Landing Page]([Link to your website])**
* **[Whitepaper (Draft/v1.0)]([Link to Whitepaper if public])**
* **[Join our Telegram Community]([Link to Telegram group])**

## Architecture Overview (Balanced Start - 11 Canisters)

The backend architecture follows the 11-canister model, optimized for initial deployment and future scalability:

1.  **`IntegrationGateway`**: Main entry point for external systems (incl. ProviderRegistry logic).
2.  **`CryptoManagerBridge`**: Specialized bridge for Crypto Manager API integration.
3.  **`ChallengeGeneration`**: Generates cryptographic challenges (CSKD).
4.  **`SignatureVerification`**: Verifies client-side signatures (CSKD, with caching).
5.  **`RegistryDirectory`**: Manages and routes to `UserRegistry` shards.
6.  **`UserRegistry`**: Manages user profiles, verification keys, encrypted Auth Shard (Starts with 1 shard).
7.  **`StorageDirectory`**: Manages and routes to `EncryptedShardStorage` shards.
8.  **`EncryptedShardStorage`**: Stores encrypted Credential/Recovery shards (Starts with 1 shard).
9.  **`SessionManager`**: Manages direct and federated sessions.
10. **`BlockchainAccessManager`**: Coordinates shard access, handles basic delegation.
11. **`SecurityServices`**: Consolidated Audit logging (AuditLog) and Account Recovery (RAS).

## Technology Stack

* **Blockchain:** Internet Computer (ICP)
* **Language (Canisters):** Rust
* **Interfaces:** Candid
* **Key Concepts:** Client-Side Key Derivation (CSKD), Shamir's Secret Sharing (SSS), Secure Account Recovery (RAS), Account Abstraction (AA - planned), Multi-Chain Integration.

## Current Status & Progress (as of end Q1 2025 / start Q2 2025)

* The DAMS backend is under active development on ICP/Rust (in private branches).
* **Milestones Achieved (Q1 2025):** Mobile Web v1 UI, Initial Solana & TON integration (core functions), AI Trending engine (Solana) built, Landing Page launched, Core DAMS implementation started, Initial DEX integration research/work started, Telegram login integration started.
* **Next Steps (Q2 2025):** Finalizing and stabilizing DAMS/RAS implementation (TOP PRIORITY), Launching Fiat On-Ramp, Launching Telegram Mini-App (Beta), Publishing Whitepaper (Draft), **Launching Closed Beta (Target: End of Q2)**.
* See our full [Roadmap](https://kongwallet.io/#roadmap) for more details.

## Examples (Coming Soon!)

In the future, we plan to add **public code examples** here or in separate repositories within the `kongsolutions` organization. These examples will demonstrate our approaches and skills in using ICP and Rust within the DAMS context.

* [Example 1: Description] - *(Coming Soon)*
* [Example 2: Description] - *(Coming Soon)*

## Team

KONGWallet is being built by a dedicated **full-time team of 6** (1 founder, 5 full-time developers) with expertise in Rust, Frontend (React Native/React/Next.JS), Backend (Nest.JS, Typescript), ML, and secure systems.

* **Martin** - [CTO/Lead DEV]
