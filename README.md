# 🛡️ AEGIS Swarm: The Real-Time Web3 Firewall

**Team:** Lone Catalyst
**Track:** National Solana Hackathon powered by Decentrathon 

---

## 📖 Overview
In Web3, smart contracts are completely static. When a hacker attacks, the contract just sits there and gets drained. Current security solutions like audits are reactive—they tell you what went wrong *after* you lost millions. 

**AEGIS** changes the paradigm. It is an active, real-time Web3 Firewall designed to secure high-risk DeFi transactions on Solana. AEGIS intercepts transactions in real-time, evaluates risk using a 5-Node AI Swarm Cluster before finality, and automatically enforces on-chain quarantines to block malicious actors.

## 🚀 Key Features

* ⚡ **Real-Time RPC Interception:** Captures live transactions via Solana Devnet WebSocket (`logsSubscribe`) and acts within milliseconds.
* 🧠 **5-Node AI Swarm Consensus:**
    * `[Alpha]` **ML Core:** Isolation Forest trained on 50k+ synthetic and public transactions to detect volume anomalies.
    * `[Beta]` **Behavior Analysis:** Detects high-frequency spam and bot behaviors.
    * `[Gamma]` **Graph Clustering:** Traces connections to known Mixer/TOR nodes.
    * `[Delta]` **Reputation System:** Checks wallets against Global PDA Blacklists.
    * `[Epsilon]` **Bytecode Scanner:** Identifies Flash-loan and Reentrancy exploit signatures.
* 🔒 **On-Chain Enforcement:** Automatically writes malicious actors to an Anchor PDA Blacklist to reject transactions before the exploit is finalized.
* ⚖️ **Progressive Risk Control:** Applies Rate Limits for medium risk, and 24h Quarantines for critical risk (drainers/flash-loans).

## 🛠️ Architecture & Tech Stack

1.  **Frontend (Command Center):** Next.js, React, Tailwind CSS. Provides a live dashboard for monitoring RPC streams and attack simulations.
2.  **Backend (Swarm Engine):** Python, FastAPI, Scikit-Learn, `solana-py`. Handles the 50k+ transaction dataset generation and AI inference.
3.  **Smart Contract (On-Chain Layer):** Rust (Anchor Framework) deployed on Solana Devnet. Manages the Quarantine State and Threat Records. *(Located in `aegis-contract/src`)*.

## 💻 How to Run Locally

Because the backend and frontend are in the same repository, you will need two terminal instances.

**1. Start the AI Swarm Backend (Python)**
```bash
# Install dependencies
pip install fastapi uvicorn scikit-learn numpy solana solders

# Run the engine
python -m uvicorn main:app --reload
The engine will train the ML model on 50,000 data points on startup and run on port 8000.

2. Start the Command Center UI (Next.js)

Bash
# Install dependencies
npm install

# Run the dashboard
npm run dev
Open http://localhost:3000 to view the live RPC stream and custom attack builder.

Built for the future of Secure DeFi & CBDC Infrastructure on Solana.
