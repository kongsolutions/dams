# KONGWallet - Decentralized Account Management System (DAMS) Backend (v1-Integration - Expanded Architecture)

## Introduction

Welcome to the primary repository hub for KONGWallet! KONGWallet is a next-generation decentralized crypto wallet built with the clear goal of making digital asset management easy, secure, and accessible for everyone, without sacrificing decentralization or user control.

This project addresses key Web3 pain points such as the complexity of managing multiple wallets/chains, high and unpredictable gas fees, and the barriers to mainstream adoption.

Our core backend (DAMS - Decentralized Account Management System) is being developed on the **Internet Computer (ICP)** using **Rust**. It employs innovative approaches like **Client-Side Key Derivation (CSKD)** and **Shamir's Secret Sharing (SSS)** for secure account recovery (RAS - Secure Account Recovery) and relies on **DID-based Authentication (DID Auth Flow)** for secure access. The v1-Integration architecture (expanded version) focuses on decentralized management of sovereign identities (DID), cryptographic keys, and interaction with blockchain networks through a **unified facade layer** for external integrations. **Account Abstraction (AA)** integration is also planned to optimize fees via the **KONG** token.

**Status:** This project is under **active development**. The core codebase resides in **private feature branches** (in our main development repository on Bitbucket/Private GitHub). This public repository serves as the main information hub and future host for public examples/modules.

**Learn More:**

* **[KONGWallet Website/Landing Page](https://kongwallet.io/)**
* **[Follow us in Twitter](https://x.com/kongwallet)**

## Expanded Architecture Overview (v1-Integration - 16 Canister Types with Facade)

The backend architecture follows the expanded v4-Integration model with **16 main canister types**, organized into logical layers and optimized for security, scalability, and integration:

**1. Entry Points (Gateways):**

* `MainIntegrationGateway`: Main entry point for request distribution. (Modified)
* `UserGateway`: Specialized for handling end-user requests and flows. (Modified)
* `ExternalAPIGateway`: Entry point for external B2B clients targeting integrated APIs (via the facade). (**NEW**)

**2. Coordination and Integration Facade:**

* `FederatedAccountManager`: Coordinates federated processes (registration, authentication) across systems. (**NEW**)
* `ExternalAPIFacade`: Provides a unified internal interface for interacting with all external APIs, hiding complexity. (**NEW**)

**3. Specific Bridges:**

* `CryptoManagerBridge`: Specialized bridge for integration with the Crypto Manager API. (Modified, called by Facade)
* `ExternalAPIBridge`: Specialized bridge for integration with a new/Kong AI API. (**NEW**, called by Facade)

**4. Cryptographic Services:**

* `ChallengeGeneration`: Generates cryptographic challenges (nonces).
* `SignatureVerification`: Verifies cryptographic signatures (incl. DID Auth) with caching.

**5. Data Management and Storage:**

* `RegistryDirectory`: Manages and routes to `UserRegistry` shards.
* `StorageDirectory`: Manages and routes to `EncryptedShardStorage` shards.
* `UserRegistry_Shard(s)`: Stores user profiles, DID documents, and public keys (sharded).
* `EncryptedShardStorage_Shard(s)`: Securely stores encrypted recovery data (SSS shards) (sharded).

**6. Core Managers:**

* `SessionManager`: Manages user sessions and authentication levels.
* `BlockchainAccessManager`: Manages links to blockchain addresses and network interaction.
* `SecurityServices`: Consolidated services for audit logs, monitoring, and account recovery (RAS).

*(Note: The original `APIGateway` from the 13-canister model has been removed in this architecture, as its role is subsumed by `ExternalAPIGateway`)*

## Technology Stack

* **Blockchain:** Internet Computer (ICP)
* **Language (Canisters):** Rust
* **Interfaces:** Candid
* **Storage:** Stable Structures
* **Cryptography:** Ed25519, Argon2id, SHA-256
* **Key Concepts:** Decentralized Identifiers (DID), **DID-based Authentication (DID Auth Flow)**, Client-Side Key Derivation (CSKD), Shamir's Secret Sharing (SSS), Secure Account Recovery (RAS), Multi-Chain Integration, External API Facade, Account Abstraction (AA - planned), Http Outcalls

## Current Status & Progress (as of end Q1 2025 / start Q2 2025)

*Note: This section reflects the status as of Q1/Q2 2025, while the architecture described above is the newer v1-Integration model.*

* The DAMS backend is under active development on ICP/Rust (in private branches).
* **Milestones Achieved (Q1 2025):** Mobile Web v1 UI, Initial Solana & TON integration (core functions), AI Trending engine (Solana) built, Landing Page launched, Core DAMS implementation started, Initial DEX integration research/work started, Telegram login integration started.
* **Next Steps (Q2 2025):** Finalizing and stabilizing DAMS/RAS implementation (**TOP PRIORITY**), Launching Fiat On-Ramp, Launching Telegram Mini-App (Beta), Publishing Whitepaper (Draft), **Launching Closed Beta (Target: End of Q2)**.

See our full [Roadmap](https://kongwallet.io/#roadmap) for more details.

## Examples (Coming Soon!)

In the future, we plan to add **public code examples** here or in separate repositories within the [kongsolutions](<#link-to-organization>) organization. These examples will demonstrate our approaches and skills in using ICP and Rust within the DAMS context.

* `[Example 1: Description]` - (Coming Soon)
* `[Example 2: Description]` - (Coming Soon)

## Team

KONGWallet is being built by a dedicated **full-time team of 6** (1 founder, 5 full-time developers) with expertise in Rust, Frontend (React Native/React/Next.JS), Backend (Nest.JS, Typescript), ML, and secure systems.

* Martin - [Lead DEV]
