use polyfill_rs::Result;
use serde_json::Value;
use colored::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let client = reqwest::Client::new();

    // 1. SET YOUR PARAMETERS
    let condition_id = "0xae866053a1e307279ac9e4e3d0f6d9965b28c5ac60c7ba15fc6fff4e726184a0"; 
    let max_whales = 20;   // API is capped at 20
    
    // --- MANUALLY SET PRICES HERE (e.g., 80% = 0.80) ---
    let manual_yes_price = 0.80;
    let manual_no_price  = 0.20;

    println!("\n══════════════════════════════════════════════════════");
    println!("       WHALE WATCHER: TOP {} HOLDERS", max_whales);
    println!("       MARKET ID: {}", condition_id.cyan());
    println!("       MANUAL PRICES: YES ${:.2} | NO ${:.2}", manual_yes_price, manual_no_price);
    println!("══════════════════════════════════════════════════════\n");

    // 2. CONSTRUCT URL
    let url = format!(
        "https://data-api.polymarket.com/holders?market={}&limit={}&minBalance=0.01",
        condition_id, max_whales
    );

    let resp = client.get(&url).send().await.expect("Network Error");
    let data: Value = resp.json().await.expect("JSON Error");

    // 3. PARSE AND DISPLAY
    if let Some(token_groups) = data.as_array() {
        for (idx, group) in token_groups.iter().enumerate() {
            // idx 0 is YES, idx 1 is NO
            let (label, price) = if idx == 0 { 
                ("YES OUTCOME", manual_yes_price) 
            } else { 
                ("NO OUTCOME", manual_no_price) 
            };
            
            println!("--- {} ---", label.bold().yellow());
            println!("{:<45} | {:<15} | {:<15}", "HOLDER (NAME/ADDRESS)", "SHARES", "USD VALUE");
            println!("────────────────────────────────────────────────────────────────────────");

            if let Some(holders) = group["holders"].as_array() {
                if holders.is_empty() {
                    println!("      (No holders found for this side)");
                }

                for h in holders.iter().take(max_whales) {
                    let address = h["proxyWallet"].as_str().unwrap_or("N/A");
                    let username = h["userName"].as_str().unwrap_or(address);
                    
                    // The Data API returns 'amount' as a string
                    let shares = h["amount"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);

                    // CALCULATION: Shares * Your Manual Price
                    let usd_val = shares * price;

                    if shares > 0.0 {
                        println!(
                            "{:<45} | {:<15.2} | {}", 
                            username.cyan(), 
                            shares, 
                            format!("(${:.0} USD)", usd_val).green()
                        );
                    }
                }
            }
            println!("\n");
        }
    } else {
        println!("No holder data found. Check if the Condition ID is valid.");
    }

    Ok(())
}