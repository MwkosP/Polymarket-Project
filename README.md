<div align="center">

## <h1> Polymwk</h1>
### Polymarket Whale/Insider Intelligence & Automated Trading 

```text
 ██████╗  ██████╗ ██╗     ██╗   ██╗███╗   ███╗██╗    ██╗██╗  ██╗
 ██╔══██╗██╔═══██╗██║     ╚██╗ ██╔╝████╗ ████║██║    ██║██║ ██╔╝
 ██████╔╝██║   ██║██║      ╚████╔╝ ██╔████╔██║██║ █╗ ██║█████╔╝ 
 ██╔═══╝ ██║   ██║██║       ╚██╔╝  ██║╚██╔╝██║██║███╗██║██╔═██╗ 
 ██║     ╚██████╔╝███████╗   ██║   ██║ ╚═╝ ██║╚███╔███╔╝██║  ██╗
 ╚═╝      ╚═════╝ ╚══════╝   ╚═╝   ╚═╝     ╚═╝ ╚══╝╚══╝ ╚═╝  ╚═╝
</div>


(This is a polyfill-rs Fork)

A high-performance intelligence layer built on a specialized fork of **polyfill-rs**. This project extends the core library to provide advanced wallet forensics, insider detection, and real-time whale-tracking capabilities.

> **Why this fork?** This version extends `polyfill-rs` v0.2.3 to bridge the gap between high-frequency trading and deep-dive market intelligence. It is optimized for speed, decoding market data ~21% faster than standard clients.

---

## 🚀 Quickstart

Get the suite running in under 60 seconds:

```bash
# 1. Clone the repository
git clone [https://github.com/MwkosP/Polymarket-Project/tree/main](https://github.com/MwkosP/Polymarket-Project/tree/main)
cd Polymarket-Project

# 2. Run the Real-Time Whale Tracker
cargo run --example whale

# 3. Discover Fresh Insider Wallets
cargo run --example find_fresh_wallets


