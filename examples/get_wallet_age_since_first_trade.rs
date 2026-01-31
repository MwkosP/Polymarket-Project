use polyfill_rs::Result;
use serde_json::Value;
use colored::*;
use chrono::{Utc, DateTime, TimeZone};

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();

    // --- VARIABLES ---
    let user_address = "0x8413168afef7ec6b7ef214dffe8d9ed3269e652d"; 
    let min_age_days = 30; // Threshold: Alert if wallet is younger than 30 days
    // -----------------

    println!("\n══════════════════════════════════════════════════════");
    println!("       WALLET GENESIS TRACKER");
    println!("       WALLET: {}", user_address.cyan());
    println!("══════════════════════════════════════════════════════\n");

    // To find the FIRST trade, we sort by TIMESTAMP and set direction to ASC (Ascending)
    // We only need the very first item (limit=1)
    let url = format!(
        "https://data-api.polymarket.com/activity?user={}&limit=1&sortBy=TIMESTAMP&sortDirection=ASC", 
        user_address
    );

    let resp = client.get(&url).send().await?.json::<Value>().await?;

    if let Some(first_trade) = resp.as_array().and_then(|a| a.first()) {
        // Extract timestamp
        let ts_int = first_trade["timestamp"].as_i64().unwrap_or(0);
        let first_date = Utc.timestamp_opt(ts_int, 0).unwrap();
        
        // Calculate age
        let now = Utc::now();
        let duration = now.signed_duration_since(first_date);
        let days_old = duration.num_days();

        println!("🐣 First Trade Date: {}", first_date.format("%d-%m-%Y %H:%M").to_string().yellow());
        println!("⏳ Wallet Time Since First Trade:       {} days", days_old.to_string().bold());

        if days_old < min_age_days {
            println!("\n🚨 {} This wallet is NEWER than {} days!", "ALERT:".on_red().white().bold(), min_age_days);
            println!("   This is a high-signal 'Fresh' whale.");
        } else {
            println!("\n✅ {} This wallet is an OLD veteran ({} days).", "NOTICE:".on_green().white().bold(), days_old);
        }
    } else {
        println!("{}", "❓ No activity found. This wallet may have never traded.".red());
    }

    println!("\n══════════════════════════════════════════════════════\n");

    Ok(())
}