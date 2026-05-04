# FareSplit Pool

A Stellar-powered ride fare splitting system for jeepney commuters in Makati that eliminates cash disputes by enabling instant USDC settlement via Soroban smart contracts.

---

## Problem
A jeepney commuter in Makati struggles to fairly split shared ride payments with strangers, often losing money or time due to cash disagreements and lack of trust.

---

## Solution
Users join a ride pool, pay once in USDC, and a Soroban smart contract instantly splits and settles payments transparently on Stellar.

---

## Timeline
- Day 1: Smart contract + pool logic  
- Day 2: Mobile wallet integration  
- Day 3: QR onboarding + testing  
- Day 4: Demo + polish  

---

## Stellar Features Used
- USDC transfers  
- Soroban smart contracts  
- Trustlines  
- Built-in DEX (optional FX conversion)  

---

## Vision
Replace fragmented cash-based commuting payments in SEA with instant, trustless, programmable settlement rails.

---

## Prerequisites
- Rust (latest stable)  
- Soroban CLI v20+  

---

## Build
```bash
soroban contract build