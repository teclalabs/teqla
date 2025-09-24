# TEQLA

**TEQLA** is a post-quantum, *feeless* Layer 1 blockchain based on **DAG + Proof-of-Useful-Work (PoUW)** — co-created by human + AI.

> **Feeless. Post-Quantum. Useful.**

## ✨ Highlights
- **Feeless UX** – micropayments are viable.  
- **Post-Quantum Security** – Dilithium/SPHINCS+ signatures from day one.  
- **Simple, Verifiable PoUW** – lightweight deterministic useful work as anti-spam.  
- **Developer-First** – EVM compatibility, TypeScript SDK, and native indexing.  

## 📚 Documentation
- [Whitepaper v0.1](./docs/whitepaper-v0.1.md)

## 🛠 Repository Structure
```
teqla/
├─ README.md
├─ LICENSE
├─ .gitignore
├─ docs/
│  └─ whitepaper-v0.1.md
├─ core/              # Rust core (DAG + PoUW + P2P)
│  ├─ Cargo.toml
│  └─ src/
│     ├─ main.rs
│     ├─ dag.rs
│     ├─ pouw.rs
│     └─ net.rs
├─ sdk/               # TypeScript SDK for dApps
│  ├─ package.json
│  ├─ tsconfig.json
│  └─ src/index.ts
├─ wallet/            # (placeholder) web/mobile wallet
├─ infra/             # docker-compose and configs
│  └─ docker-compose.yml
└─ tests/             # chaos/integration tests
```

## 🚀 Getting Started (SDK)
```bash
cd sdk
npm install
npm run build
```

## 🔧 Getting Started (Core – Rust)
```bash
cd core
cargo run
```

## 📈 Roadmap (summary)
- MVP (6 months): DAG + PoUW + PQC + SDK + public testnet.  
- Phase 2: wallet, advanced indexing, developer incentives.  
- Phase 3: evolution to PoVU (stake + verifiable utility).  
