# 🏅 Kudos Meter (Solana Anchor Counter)

![License](https://img.shields.io/badge/license-MIT-blue.svg) ![Solana](https://img.shields.io/badge/Solana-Devnet-green) ![Anchor](https://img.shields.io/badge/Anchor-0.30.1-blueviolet)

**Kudos Meter** is a decentralized social reputation system built on Solana. It serves as a proof-of-concept for the Turbin3 Q4 Builders Cohort assignment ("Rust Counter").

Instead of a generic counter, this program tracks "Kudos" (reputation points) given to specific users, storing the count on-chain in a Program Derived Address (PDA).

---

## 🔗 Deployment Details (Devnet)

* **Program ID:** `BrComrWp8AWEy3rDDxQohg8n7M7e5mLSrwiDRbGNNKu1`
* **Solana Explorer:** [View Program on Devnet](https://explorer.solana.com/address/BrComrWp8AWEy3rDDxQohg8n7M7e5mLSrwiDRbGNNKu1?cluster=devnet)

---

## 📖 Project Concept

### Use Case
In decentralized organizations (DAOs) and developer communities, tracking contributions and "goodwill" is often difficult or centralized (like Discord roles). **Kudos Meter** solves this by allowing any user to send an on-chain acknowledgment to another user. This action permanently increments the recipient's "Kudos Counter," creating a verifiable and transparent reputation score that lives on the blockchain.

### User Story
> "As a DAO contributor, I want to send a 'Kudo' to a colleague's wallet address so that their reputation score increases transparently on-chain, proving their value to the community without relying on a centralized database."

---

## 🏗 Architecture

The system uses a **Program Derived Address (PDA)** to store the counter for each specific user. This ensures that only the `kudos_meter` program can modify the count, preventing users from faking their own reputation.

### Architectural Diagram
```mermaid
graph TD
    User(User Wallet) -->|Sign Transaction| Client(Client / Test Script)
    Client -->|RPC Call: give_kudos| Solana(Solana Cluster)
    Solana -->|Route to Program| Program[Kudos Meter Program]
    
    subgraph On-Chain Storage
        Program -->|Derive PDA| PDA[Kudos Account PDA]
        PDA -->|Read Current Count| State{Current State}
        State -->|Increment Count| NewState[Count + 1]
        NewState -->|Save| PDA
    end
    
    PDA -.->|Return| Client
⚙️ How to Run
Prerequisites
Rust

Solana CLI

Anchor CLI

Node.js & Yarn

1. Installation
Clone the repository and install dependencies:

Bash

git clone [https://github.com/YOUR_USERNAME/kudos-meter.git](https://github.com/YOUR_USERNAME/kudos-meter.git)
cd kudos_meter
yarn install
2. Build the Program
Compile the Rust code to BPF bytecode:

Bash

anchor build
3. Testing (Localnet)
To run the tests on your local machine (ensure you have solana-test-validator installed, or let Anchor handle it):

Bash

anchor test
This will spin up a local validator, deploy the program, run the initialize and giveKudos tests, and shut down.

4. Testing (Devnet)
Since the program is already deployed to Devnet, you can run the test script against the live network:

Ensure your Anchor.toml is configured for Devnet.

Run the test skipping the local validator:

Bash

anchor test --skip-local-validator
📂 Project Structure
Plaintext

kudos_meter/
├── programs/
│   └── kudos_meter/
│       └── src/
│           └── lib.rs       # Main smart contract logic (Rust)
├── tests/
│   └── kudos_meter.ts       # TypeScript integration tests
├── Anchor.toml              # Configuration for network & scripts
└── package.json             # Node dependencies
🛠 Tech Stack
Blockchain: Solana (Devnet)

Framework: Anchor

Language: Rust (Smart Contract), TypeScript (Tests/Client)

👤 Author
Bigg Manuel

Role: Developer

Cohort: Turbin3 Q4 Builders

Assignment: Rust Counter & Solana Use Case
