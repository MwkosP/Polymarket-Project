use polyfill_rs::{WebSocketStream, StreamMessage, Result, Side};
use colored::*;
use tokio_stream::StreamExt; // Required for the .next() loop
use rust_decimal::prelude::ToPrimitive; // Required for price/size math

#[tokio::main]
async fn main() -> Result<()> {
    println!("\n══════════════════════════════════════════════════════");
    println!("       WHALE TRACKER (POLYFILL-RS v0.2.3)");
    println!("══════════════════════════════════════════════════════\n");

    // 1. Concrete implementation (not the trait)
    let mut stream = WebSocketStream::connect("wss://clob.polymarket.com/ws").await?;

    // 2. Subscribe to the global activity channel
    stream.subscribe_activity().await?;

    println!("📡 Live Stream Active. Waiting for big moves...");

    while let Some(msg) = stream.next().await {
        // 3. Pattern Match using the Struct style { data }
        if let StreamMessage::Trade { data: trade } = msg {
            let price = trade.price.to_f64().unwrap_or(0.0);
            let size = trade.size.to_f64().unwrap_or(0.0);
            let value = price * size;

            if value >= 5000.0 {
                let side_label = if trade.side == Side::BUY { 
                    "BUY".green().bold() 
                } else { 
                    "SELL".red().bold() 
                };

                println!(
                    "🐋 Whale Found: {} | {} ${:.2}", 
                    trade.taker.cyan(), 
                    side_label, 
                    value
                );
                println!("   Asset:  {}", trade.asset_id.dimmed());
                println!("------------------------------------------------------");
            }
        }
    }

    Ok(())
}