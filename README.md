<div align="center">


```text
 ██████╗  ██████╗ ██╗     ██╗   ██╗███╗   ███╗██╗    ██╗██╗  ██╗      ██████╗ ███████╗
 ██╔══██╗██╔═══██╗██║     ╚██╗ ██╔╝████╗ ████║██║    ██║██║ ██╔╝      ██╔══██╗██╔════╝
 ██████╔╝██║   ██║██║      ╚████╔╝ ██╔████╔██║██║ █╗ ██║█████╔╝ █████╗██████╔╝███████╗
 ██╔═══╝ ██║   ██║██║       ╚██╔╝  ██║╚██╔╝██║██║███╗██║██╔═██╗ ╚════╝██╔══██╗╚════██║
 ██║     ╚██████╔╝███████╗   ██║   ██║ ╚═╝ ██║╚███╔███╔╝██║  ██╗      ██║  ██║███████║
 ╚═╝      ╚═════╝ ╚══════╝   ╚═╝   ╚═╝     ╚═╝ ╚══╝╚══╝ ╚═╝  ╚═╝      ╚═╝  ╚═╝╚══════╝
```
[![Crates.io](https://img.shields.io/crates/v/YOUR_CRATE_NAME.svg)](https://crates.io/crates/YOUR_CRATE_NAME)
[![Docs](https://docs.rs/YOUR_CRATE_NAME/badge.svg)](https://docs.rs/YOUR_CRATE_NAME)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)


Insider Intelligence & Whale Forensics for Polymarket
</div>
<br>
<hr>

This is a polyfill-rs Fork. <br><br>

While poly-fill-rs provides lightning speed gateways for automated trading(HFT), polymwk-rs is engineered specifically for analysts and developers who need to look beneath the surface of simple trades and Arbitrage and see Historical Patterns and Statistics.
<br><br>

### Why polymwk-rs?
Wallet Intelligence
- `find_fresh_wallets` — detect newly active wallets
- `get_wallet_age`
- `get_wallet_age_since_first_trade`
- `get_user_identity`
- `get_username`
- `get_user_winrate`
- `get_all_users_traded_markets_number`
- `top_holders` — identify concentrated positions

User Activity Tracking
- `get_last_trades`
- `get_current_user_active_markets`
- `get_current_user_active_events`

### Orderbook & Market State
- `get_orderbook_dynamic`
- `get_orderbook_static`

### Event/Markets Monitoring
- `get_events`

<br>Many more to be added soon..
<br>


## Quick Start

To Download:

```cmd
git clone https://github.com/MwkosP/polymwk-rs.git
cd polymwk-rs
```

To quickly run some examples showcasing Capabilities:

```cmd
cargo run --example get_wallet_age
```

To use like a Library: Add to your `Cargo.toml`:

```toml
[dependencies]
polymwk-rs = "0.0.1"
```

Replace your imports:

```rust
// Before: use polymarket_rs_client::{ClobClient, Side, OrderType};
use polymwk_rs::{ClobClient, Side, OrderType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClobClient::new("https://clob.polymarket.com");
    let markets = client.get_sampling_markets(None).await?;
    println!("Found {} markets", markets.data.len());
    Ok(())
}
```


