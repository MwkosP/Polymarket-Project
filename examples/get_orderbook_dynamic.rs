use polyfill_rs::Result;
use serde_json::Value;
use colored::*;
use std::time::Duration; // Added for the timer
use tokio::time::interval; // Added for the loop

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let client = reqwest::Client::new();

    // Token ID(Either Yes or No Token ID)
    let token_id = "31909393856053520018507280554368586128735607865481470209833276228711603596664"; 

    // Create an interval timer (e.g., 2 seconds)
    let mut ticker = interval(Duration::from_secs(5));

    loop {
        // Wait for the next tick
        ticker.tick().await;

        let url = format!("https://clob.polymarket.com/book?token_id={}", token_id);
        let resp = match client.get(&url).send().await {
            Ok(r) => r,
            Err(_) => continue, // Skip if network fails
        };
        
        let book: Value = resp.json().await.expect("JSON Error");

        // Clear the screen (ANSI escape code) to make it look like a dashboard
        print!("{}[2J{}[1;1H", 27 as char, 27 as char);

        println!("\n══════════════════════════════════════════════════════");
        println!("       LIVE ORDERBOOK DEPTH (Refreshing every 2s)");
        println!("       TOKEN: {}", token_id.cyan());
        println!("══════════════════════════════════════════════════════\n");

        println!("{:<15} | {:<15}", "PRICE".bold(), "SIZE (Shares)".bold());
        println!("──────────────────────────────────────────────────────");

        if let Some(asks) = book["asks"].as_array() {
            println!("{}", "--- ASKS (SELLERS) ---".red());
            for ask in asks.iter().take(5).rev() {
                let p = ask["price"].as_str().unwrap_or("0.00");
                let s = ask["size"].as_str().unwrap_or("0");
                println!("  {:<15} | {:<15}", p.red(), s);
            }
        }

        println!("---------- MIDPOINT ----------");

        if let Some(bids) = book["bids"].as_array() {
            for bid in bids.iter().take(5) {
                let p = bid["price"].as_str().unwrap_or("0.00");
                let s = bid["size"].as_str().unwrap_or("0");
                println!("  {:<15} | {:<15}", p.green(), s);
            }
            println!("{}", "--- BIDS (BUYERS) ---".green());
        }

        println!("\n──────────────────────────────────────────────────────");
        
        if let (Some(best_bid), Some(best_ask)) = (book["bids"][0]["price"].as_str(), book["asks"][0]["price"].as_str()) {
            let bid_f: f64 = best_bid.parse().unwrap_or(0.0);
            let ask_f: f64 = best_ask.parse().unwrap_or(0.0);
            let spread = ask_f - bid_f;
            let spread_str = format!("{:.4}", spread).yellow();
            
            println!("Spread: {} | Best Bid: {} | Best Ask: {}", spread_str, best_bid.green(), best_ask.red());
        }
    }
}