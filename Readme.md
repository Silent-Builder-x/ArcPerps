# ArcPerps: Privacy-Preserving Perpetual Exchange

## 🌟 Introduction

ArcPerps is a next-generation decentralized perpetual futures protocol built on **Arcium** and **Solana**. By leveraging **Secure Multi-Party Computation (MPC)**, ArcPerps ensures that all trading data—including positions, orders, and liquidation checks—remains confidential. This eliminates adversarial strategies like liquidation sniping and MEV, fostering a fair and secure trading environment.

## 🚀 Key Features

### 🔒 Privacy by Design
- **Encrypted Positions:** Trader data is encrypted and secret-shared using x25519, ensuring complete confidentiality.
- **Blind Risk Engine:** All computations, including PnL and liquidation checks, are performed in the encrypted domain.

### ⚡ High Performance
- **Solana Integration:** Built on Solana’s high-speed blockchain, ArcPerps supports high-frequency trading with minimal latency.
- **Dynamic Liquidation Checks:** Real-time risk assessments ensure accurate and fair liquidation decisions.

### 🌍 Real-World Impact
- **Fair Trading:** Prevents MEV and liquidation sniping, leveling the playing field for all traders.
- **Institutional Adoption:** Privacy and security features meet the needs of institutional traders, driving deeper liquidity.

## 🛠 Technical Overview

### Core Components
1. **Confidential Position Management:** Positions are encrypted and stored securely on-chain.
2. **Encrypted Risk Engine:** The `check_risk` function calculates PnL and liquidation status without exposing sensitive data.
3. **Event-Driven Architecture:** Risk events are emitted for real-time monitoring and response.

### Build Instructions
```bash
# Compile Arcis circuits and Solana program
arcium build

# Deploy to Cluster 456
arcium deploy --cluster-offset 456 --recovery-set-size 4 --keypair-path ~/.config/solana/id.json -u d
```

### Deployment Status
The protocol is fully deployed and operational on the **v0.8.3 Arcium Devnet**, showcasing its readiness for real-world use cases.

## 🧠 Why ArcPerps?

### Innovation
ArcPerps redefines decentralized trading by integrating Arcium’s MPC technology, ensuring unparalleled privacy and fairness.

### Technical Excellence
The protocol combines advanced cryptographic techniques with Solana’s scalability, delivering a robust and efficient trading solution.

### User-Centric Design
A seamless and intuitive interface ensures that traders can focus on strategy without worrying about security.

## 📄 Open Source Contribution

ArcPerps is fully open-source and welcomes contributions from the community. Together, we can push the boundaries of decentralized trading.

[GitHub Repository](https://github.com/silent-builder-x/ArcPerps)