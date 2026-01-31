use polyfill_rs::Result;
use serde_json::Value;
use colored::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let client = reqwest::Client::new();

    // 1. SET YOUR PARAMETERS
    let condition_id = "0xae866053a1e307279ac9e4e3d0f6d9965b28c5ac60c7ba15fc6fff4e726184a0"; 
    let max_whales = 100;   
    let min_hold = "0.01"; 

    // --- MANUALLY SET THE PRICE HERE ---
    let current_price = 0.80; 
    let no_price = 1.0 - current_price;

    // --- FETCH TOTAL VOLUME FOR PERCENTAGE CALCULATION ---
    let gamma_url = format!("https://gamma-api.polymarket.com/markets?condition_id={}", condition_id);
    let g_resp = client.get(&gamma_url).send().await?.json::<Value>().await?;
    let total_volume: f64 = g_resp[0]["volume"]
        .as_str()
        .unwrap_or("0")
        .parse()
        .unwrap_or(1.0); 

    println!("\n══════════════════════════════════════════════════════");
    println!("       TOP {} HOLDERS", max_whales);
    println!("       MARKET ID: {}", condition_id.cyan());
    println!("       SET PRICES: YES ${:.2} | NO ${:.2}", current_price, no_price);
    println!("       TOTAL SHARES: {:.0}", total_volume);
    println!("══════════════════════════════════════════════════════\n");

    // 2. CONSTRUCT URL
    let url = format!(
        "https://data-api.polymarket.com/holders?market={}&limit={}&minBalance={}",
        condition_id, max_whales, min_hold
    );

    let resp = client.get(&url).send().await.expect("Network Error");
    let data: Value = resp.json().await.expect("JSON Error");

    // 3. PARSE AND DISPLAY
    if let Some(token_groups) = data.as_array() {
        for (idx, group) in token_groups.iter().enumerate() {
            let (label, price_to_use) = if idx == 0 { 
                ("YES OUTCOME", current_price) 
            } else { 
                ("NO OUTCOME", no_price) 
            };
            
            println!("--- {} ---", label.bold().yellow());
            println!("{:<45} | {:<15} | {:<15}", "HOLDER (NAME/ADDRESS)", "SHARES", "USDC VALUE / OWN %");
            println!("──────────────────────────────────────────────────────────────────────────────");

            if let Some(holders) = group["holders"].as_array() {
                for h in holders.iter().take(max_whales) {
                    let address = h["proxyWallet"].as_str().unwrap_or("N/A");
                    let username = h["userName"].as_str().unwrap_or(address);
                    
                    let shares = if let Some(s) = h["amount"].as_str() {
                        s.parse::<f64>().unwrap_or(0.0)
                    } else {
                        h["amount"].as_f64().unwrap_or(0.0)
                    };

                    let usd_val = shares * price_to_use;
                    let percent_owned = (shares / total_volume) * 100.0;

                    if shares > 0.0 {
                        println!(
                            "{:<45} | {:<15.2} | {}  {}", 
                            username.cyan(), 
                            shares, 
                            format!("(${:.0} USD)", usd_val).green(),
                            format!("{:.2}%", percent_owned).magenta() // Percentage added next to USD
                        );
                    } else {
                        println!("{:<45} | {:<15} (Raw: {:?})", username.dimmed(), "0.00".red(), h["amount"]);
                    }
                }
            }
            println!("\n");
        }
    } else {
        println!("No holder data found. Check if the Condition ID is correct.");
    }

    Ok(())
}