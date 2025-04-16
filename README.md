# KONGWallet - Decentralized Account Management System (DAMS)

## Introduction

Welcome to the primary repository hub for KONGWallet! KONGWallet is a next-generation decentralized crypto wallet built with the clear goal of making digital asset management easy, secure, and accessible for everyone, without sacrificing decentralization or user control.

This project addresses key Web3 pain points such as the complexity of managing multiple wallets/chains, high and unpredictable gas fees, and the barriers to mainstream adoption.

Our core backend (DAMS - Decentralized Account Management System) is being developed on the **Internet Computer (ICP)** using **Rust**. It employs innovative approaches like **Client-Side Key Derivation (CSKD)** and **Shamir's Secret Sharing (SSS)** for secure account recovery (RAS - Recovery Account System) and relies on **DID-based Authentication (DID Auth Flow)** for secure access. The v1-Integration architecture focuses on decentralized management of sovereign identities (DID), cryptographic keys, and interaction with blockchain networks through a **unified facade layer** for external integrations. **Account Abstraction (AA)** integration is also planned to optimize fees via the **KONG** token.

**Status:** This project is under **active development**. The core codebase resides in **private feature branches** (in our main development repository on Bitbucket). This public repository serves as the main information hub and future host for public modules.

**Learn More:**

* **[KONGWallet Website](https://kongwallet.io/)**
* **[Follow us on Twitter](https://x.com/kongwallet)**

## Architecture Overview (v1-Integration)

The backend architecture follows the expanded v1-Integration model with **16 main canister types**, organized into logical layers and optimized for security, scalability, and integration:

### 1. Entry Points (Gateways)

* `MainIntegrationGateway`: Main entry point for request distribution
* `UserGateway`: Specialized for handling end-user requests and flows
* `ExternalAPIGateway`: Entry point for external B2B clients targeting integrated APIs

### 2. Coordination and Integration Facade

* `FederatedAccountManager`: Coordinates federated processes (registration, authentication) across systems
* `ExternalAPIFacade`: Provides a unified internal interface for interacting with all external APIs

### 3. Specific Bridges

* `CryptoManagerBridge`: Specialized bridge for integration with the Crypto Manager API
* `ExternalAPIBridge`: Specialized bridge for integration with new/Kong AI API

### 4. Cryptographic Services

* `ChallengeGeneration`: Generates cryptographic challenges (nonces)
* `SignatureVerification`: Verifies cryptographic signatures (incl. DID Auth) with caching

### 5. Data Management and Storage

* `RegistryDirectory`: Manages and routes to `UserRegistry` shards
* `StorageDirectory`: Manages and routes to `EncryptedShardStorage` shards
* `UserRegistry_Shard(s)`: Stores user profiles, DID documents, and public keys (sharded)
* `EncryptedShardStorage_Shard(s)`: Securely stores encrypted recovery data (SSS shards)

### 6. Core Managers

* `SessionManager`: Manages user sessions and authentication levels
* `BlockchainAccessManager`: Manages links to blockchain addresses and public data
* `SecurityServices`: Consolidated services for audit logs, monitoring, and account recovery (RAS)

## Technology Stack

* **Blockchain:** Internet Computer (ICP)
* **Language (Canisters):** Rust
* **Interfaces:** Candid
* **Storage:** Stable Structures
* **Cryptography:** Ed25519, Argon2id, SHA-256
* **Key Concepts:** Decentralized Identifiers (DID), DID-based Authentication, Client-Side Key Derivation (CSKD), Shamir's Secret Sharing (SSS), Recovery Account System (RAS), Multi-Chain Integration, External API Facade, Account Abstraction (AA - planned), Http Outcalls

## Beta Features (Phase 1)

### I. Account Management and Security
1. **Registration:** Create non-custodial account via:
   * Email and Password (using SRP)
   * Google account (OAuth)
   * Telegram account (OAuth / Login Widget)
2. **Seed Phrase:** Generated during registration; Displayed for backup after first login (part of Onboarding)
3. **Internal Storage:** SSS splitting into 3 shards, encryption with KDF key (from Password/PIN+Salt), storage of encrypted shards on IC
4. **Login:** Sign in with Email/Password (SRP), Google, or Telegram; Retrieve 2 encrypted shards + salt from IC; Local decryption and SSS reconstruction of Seed Phrase in frontend
5. **API Access Token:** Automatic JWT token retrieval after successful login/registration for Crypto Manager API access
6. **Account Recovery (RAS):**
   * Primary method: By entering **complete Seed Phrase + 2FA** verification
   * Password Reset (SRP login only): Via email verification (doesn't restore key access)
7. **Logout:** Ability to exit session

### II. Portfolio Management
1. **Dashboard:**
   * Balance overview **for SOL, TON** and related SPL/Jetton tokens
   * Display of amounts and **approximate USD value**
   * Display of total portfolio value in USD
2. **AI Trending Integration:**
   * Display data for **top 10 trending tokens on Solana network**

### III. Basic Transactions
1. **Receive:** Display SOL and TON address / QR code
2. **Send:** Send SOL or TON to:
   * Blockchain address (SOL/TON)
   * Registered Email in KONGWallet
   * Registered Telegram username in KONGWallet
3. **Local Signing and Broadcasting:** Transactions are signed locally in frontend and broadcast directly from frontend (to drpc.org)
4. **Status Tracking:** Backend receives hash and tracks; frontend displays basic status (Pending/Confirmed/Failed)

### IV. Swap
1. **Swap Interfaces:** Quick Swap and Manual Swap (with slippage option)
2. **Supported Exchanges:**
   * SOL <> SPL tokens (via **Jupiter**)
   * TON <> Jetton tokens (via **STON.fi / Omniston Protocol**)
3. **Execution:** Get quotes from aggregators; Local signing in frontend; Direct broadcast from frontend; Status tracking

### V. User Experience and Onboarding
1. **"Get Started" Tasks:** Display and track completion of tasks: "Backup Seed Phrase", "Receive SOL/TON First Time", "Start Trading (Swap)"
2. **Interface:** Access via **Mobile Web Browser** and **Telegram Mini-App** (showing Mobile Web interface through basic bot)

### VI. Infrastructure
1. **Core Components:** Configured and operational IC canisters, K8s cluster, NestJS microservices (Crypto Manager API), Kong Gateway (with JWT validation), Databases, RabbitMQ, Redis, CI/CD, Monitoring, Logging, drpc.org integration

## Transition Strategy: Beta to Full Version

Our transition strategy ensures seamless migration from Beta to the full version through:

1. **Architecture Preparation in Beta Phase**
   * Versioned API interfaces
   * Abstract layers for components that will change
   * Clean separation of business logic from protocol-specific logic

2. **Phased Sharding Implementation**
   * Preparation for sharding with directory mechanisms
   * Gradual introduction of dedicated directory canisters
   * Progressive data redistribution during scaling

3. **Phased Security Feature Implementation**
   * Infrastructure preparation between Beta and full version
   * WebAuthn/Passkeys as optional method first, then required for new registrations
   * Enhanced recovery system with Time Lock and social recovery

4. **Data Migration and Compatibility**
   * Version flags in user records
   * Group-based user migration
   * Backward compatibility during transition
   * "Double-write" during transition phase

5. **Canister Expansion Plan**
   * Phase 1: After Beta stabilization (10-12 canisters)
   * Phase 2: Full implementation (14-16 canisters)
   * Monitoring and scaling based on defined thresholds

6. **Zero Downtime Techniques**
   * "Dual canister" technique for seamless updates
   * Rolling upgrade strategy for sharded components
   * Gradual traffic routing between versions

## Current Status and Progress (Q2 2025)

* The DAMS backend is under active development on ICP/Rust (in private branches)
* **Milestones Achieved (Q1 2025):** Mobile Web v1 UI, Initial Solana & TON integration (core functions), AI Trending engine (Solana) built, Landing Page launched, Core DAMS implementation started, Initial DEX integration research/work started, Telegram login integration started
* **Next Steps (Q2 2025):** Finalizing and stabilizing DAMS/RAS implementation (**TOP PRIORITY**), Launching Telegram Mini-App (Beta), Publishing Whitepaper (Draft), **Launching Closed Beta (Target: End of Q2/Begining of Q3)**

See our full [Roadmap](https://kongwallet.io/#roadmap) for more details.

## Team

KONGWallet is being built by a dedicated **full-time team of 6** (1 founder, 4 full-time developers) with expertise in Rust, Frontend (React Native/React/Next.JS), Backend (Nest.JS, Typescript), ML, and secure systems.


* Martin - [Lead DEV]
