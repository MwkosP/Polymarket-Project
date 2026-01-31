use polyfill_rs::Result;
use serde_json::Value;
use colored::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let client = reqwest::Client::new();

    // Token ID(Either Yes or No Token ID)
    let token_id = "93227524655023659661897136914734275757133748893766260009691184935571133544792"; 

    println!("\n══════════════════════════════════════════════════════");
    println!("       FETCHING LIVE ORDERBOOK DEPTH");
    println!("══════════════════════════════════════════════════════\n");

    let url = format!("https://clob.polymarket.com/book?token_id={}", token_id);
    let resp = client.get(&url).send().await.expect("Network Error");
    let book: Value = resp.json().await.expect("JSON Error");

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
    
    // THE FIX: Parse prices and convert the result back to String for coloring
    if let (Some(best_bid), Some(best_ask)) = (book["bids"][0]["price"].as_str(), book["asks"][0]["price"].as_str()) {
        let bid_f: f64 = best_bid.parse().unwrap_or(0.0);
        let ask_f: f64 = best_ask.parse().unwrap_or(0.0);
        let spread = ask_f - bid_f;
        
        // Use format! to get the spread as a string, then color it
        let spread_str = format!("{:.4}", spread).yellow();
        
        println!("Spread: {} | Best Bid: {} | Best Ask: {}", spread_str, best_bid.green(), best_ask.red());
    }

    Ok(())
}