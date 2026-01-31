# polyfill-rs
*(Experimental extensions for whale & event monitoring)*
THIS IS A polyfill-rs Fork extending its usecases and outdated functions(Still under heavy construction!!)


[![Crates.io](https://img.shields.io/crates/v/polyfill-rs.svg)](https://crates.io/crates/polyfill-rs)
[![Documentation](https://docs.rs/polyfill-rs/badge.svg)](https://docs.rs/polyfill-rs)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

A **high-performance, API-compatible drop-in replacement** for `polymarket-rs-client`, focused on **low latency**, **zero-allocation hot paths**, and **production-grade trading workloads**.

This fork additionally introduces **experimental analytics and monitoring APIs** aimed at:
- **Whale tracking**
- **User behavior analysis**
- **Orderbook & event monitoring**
- **Crypto-focused Polymarket bets**

> ⚠️ **Experimental Notice**  
> Several advanced endpoints are **still experimental** and subject to change.  
> More features will be added as Polymarket’s data surface evolves.

---

## Why polyfill-rs?

- **100% API-compatible** with `polymarket-rs-client`
- Identical method signatures — swap imports and go
- Latency-optimized data structures
- Zero-allocation critical paths
- SIMD-accelerated JSON parsing
- Tuned HTTP/2 networking for Polymarket payloads

---

## Experimental Analytics & Monitoring (New)

This fork adds **non-trading analytical endpoints** designed for **market intelligence**, **whale detection**, and **event surveillance**, especially for **crypto-related markets**.

### 🐋 Wallet & User Intelligence
- `find_fresh_wallets` — detect newly active wallets
- `get_wallet_age`
- `get_wallet_age_since_first_trade`
- `get_user_identity`
- `get_username`
- `get_user_winrate`
- `get_all_users_traded_markets_number`
- `top_holders` — identify concentrated positions

### 📊 User Activity Tracking
- `get_last_trades`
- `get_current_user_active_markets`
- `get_current_user_active_events`

### 📈 Orderbook & Market State
- `get_orderbook_dynamic`
- `get_orderbook_static`

### 🗓 Event Monitoring
- `get_events`

These endpoints are **read-heavy, analytics-oriented**, and optimized for:
- Whale movement alerts
- Suspicious activity detection
- Event-level liquidity monitoring
- Strategy research & automation

> Expect breaking changes in these APIs while they mature.

---

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
polyfill-rs = "0.2.3"

