use polyfill_rs::Result;
use serde_json::Value;
use colored::*;
use chrono::{DateTime, Utc, TimeZone};

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();

    // --- SETTINGS ---
    let user_address = "0x9d4b905c57f466005ef13ac04693c4653030b8e6"; 
    let last_trades = 100; 

    let url = format!(
        "https://data-api.polymarket.com/activity?user={}&limit={}", 
        user_address, 
        last_trades
    );

    let resp = client.get(&url).send().await.expect("Network Error");
    let activity: Value = resp.json().await.expect("JSON Error");

    println!("\n════════════════════════════════════════════════════════════════════════════════════════");
    println!("       WHALE ACTIVITY TRACKER | LAST {} TRADES", last_trades);
    println!("       WALLET: {}", user_address.cyan());
    println!("════════════════════════════════════════════════════════════════════════════════════════\n");

    if let Some(items) = activity.as_array() {
        // Table Header
        println!("{:<18} | {:<7} | {:<5} | {:<8} | {:<10} | {:<10} | {}", 
            "DATE (LOCAL)", "SIDE", "BET", "PRICE", "SHARES", "TOTAL USD", "MARKET"
        );
        println!("────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────");

        for item in items {
            // 1. DATE: Using proxyTimestamp or timestamp
            let raw_ts = item["proxyTimestamp"].as_str().or(item["timestamp"].as_str()).unwrap_or("");
            let date_raw = DateTime::parse_from_rfc3339(raw_ts)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            
            // Convert to Local (Greece)
            let date_display = date_raw.with_timezone(&chrono::Local).format("%d-%m-%Y %H:%M").to_string();

            // 2. TYPE & SIDE
            let act_type = item["type"].as_str().unwrap_or("");
            let side = item["side"].as_str().unwrap_or("");
            let outcome = item["outcome"].as_str().unwrap_or("");
            
            // SIDE LOGIC: Identify BUY, SELL, SPLIT, or MERGE
            let side_display = if act_type == "SPLIT" {
                "ACT(S)".white().dimmed()
            } else if act_type == "MERGE" {
                "ACT(M)".white().dimmed()
            } else {
                match side {
                    "BUY" => "BUY".green().bold(),
                    "SELL" => "SELL".red().bold(),
                    _ => "ACT".white().dimmed(),
                }
            };

            // BET LOGIC: YES / NO
            let outcome_display = match outcome {
                "Yes" => "YES".magenta(),
                "No" => "NO".blue(),
                _ => "--".white().dimmed(),
            };

            // 3. PRICE
            let price = item["price"].as_f64().unwrap_or(0.0);
            let price_display = if price > 0.0 { 
                format!("{:.0}¢", price * 100.0).yellow() 
            } else { 
                "--".white().dimmed() 
            };

            // 4. SHARES
            let shares = item["tokens"].as_str()
                .and_then(|s| s.parse::<f64>().ok())
                .or(item["size"].as_f64())
                .unwrap_or(0.0);

            // 5. TOTAL USD
            let total_usd = item["cash"].as_f64()
                .or(item["usdcSize"].as_f64())
                .unwrap_or(shares * price);

            let title = item["title"].as_str().unwrap_or("Unknown Market");

            if shares > 0.0 || total_usd > 0.0 {
                println!(
                    "{:<18} | {:<7} | {:<5} | {:<8} | {:<10.1} | ${:<9.2} | {}",
                    date_display.dimmed(),
                    side_display,
                    outcome_display,
                    price_display,
                    shares,
                    total_usd,
                    title.cyan()
                );
            }
        }
    }
    Ok(())
}