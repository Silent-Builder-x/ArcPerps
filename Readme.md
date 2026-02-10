# ArcPerps: FHE-Powered Anti-Manipulation Perpetual Exchange

## 📉 Overview

**ArcPerps** is a privacy-preserving perpetual futures protocol built on **Arcium** and **Solana**.

Current decentralized perpetual exchanges (Perp DEXs) suffer from "Transparency Bias." Because all positions, entry prices, and liquidation points are visible on-chain, large traders are frequently targeted by **Liquidation Sniping** bots and MEV searchers. **ArcPerps** utilizes **Fully Homomorphic Encryption (FHE)** to move the entire risk engine—including position management, PnL calculation, and liquidation checks—into a confidential computation layer.

## 🚀 Live Deployment Status (Verified)

The protocol is fully operational and verified on the Arcium Devnet.

- **MXE Address:** `ESBrinp1cQCXq1X4MabPh5zXUY9Z2vZPiHXn66MEzEcn`
- **MXE Program ID:** `5mjjZpqBn7QCQKiVYY8LraZC7dDC8pHACMvHHCQ7UtLc`
- **Computation Definition:** `EEXK2RELZfcG1Lh2qMb5dcvsB25PNfG7VjzX9gySQ3ry`
- **Status:** `Active`

## 🧠 Core Innovation: The "Blind" Risk Engine

ArcPerps implements a **Confidential Derivatives Core** using Arcis FHE circuits:

- **Shielded Positions:** Trader leverage and entry prices are encrypted locally using x25519 before being sent to the cluster.
- **Homomorphic PnL Calculation:** The engine computes unrealized PnL (uPnL) and Equity entirely in the ciphertext space: $Equity = Collateral + \frac{(MarkPrice - EntryPrice) \times Size}{EntryPrice}$.
- **MEV Resistance:** By hiding position proximity to liquidation, ArcPerps prevents predatory "Liquidation Sniping" and front-running orders, ensuring a fairer trading environment for institutions and whales.

## 🛠 Build & Implementation

```
# Compile Arcis circuits and Solana program
arcium build

# Deploy to Cluster 456
arcium deploy --cluster-offset 456 --recovery-set-size 4 --keypair-path ~/.config/solana/id.json -u d

```

## 📄 Technical Specification

- **Engine:** `check_perp_liquidation` (Arcis-FHE Circuit)
- **Security:** Supported by Arcium's Multi-Party Execution and Recovery Set (Size 4).
- **Compliance:** Built following **Internal V4 Standards** with verified `/// CHECK:` safety comments for Anchor IDL.