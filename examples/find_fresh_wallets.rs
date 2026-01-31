use polyfill_rs::Result;
use serde_json::Value;
use colored::*;
use chrono::{DateTime, Utc};

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::builder()
        .user_agent("WhaleScanner/3.0")
        .build()?;

    // --- YOUR SPECIFIED DAYS ---
    let days_since_creation = 5; 
    // ---------------------------

    println!("\n══════════════════════════════════════════════════════");
    println!("       SEARCHING FOR WALLETS CREATED WITHIN {} DAYS", days_since_creation);
    println!("══════════════════════════════════════════════════════\n");

    // We pull the most recent 100 global trades to find active addresses
    let trades_url = "https://data-api.polymarket.com/trades?limit=100";
    let resp = client.get(trades_url).send().await?.json::<Value>().await?;

    let now = Utc::now();
    let mut seen = std::collections::HashSet::new();
    let mut found_count = 0;

    if let Some(trades) = resp.as_array() {
        for trade in trades {
            let address = trade["taker"].as_str().unwrap_or("");
            if address.is_empty() || !seen.insert(address.to_string()) {
                continue;
            }

            // Check the profile of this active user
            let profile_url = format!("https://gamma-api.polymarket.com/public-profile?address={}", address);
            if let Ok(profile_resp) = client.get(&profile_url).send().await {
                if let Ok(profile) = profile_resp.json::<Value>().await {
                    
                    if let Some(created_str) = profile["createdAt"].as_str() {
                        if let Ok(created_at) = DateTime::parse_from_rfc3339(created_str) {
                            let created_at_utc = created_at.with_timezone(&Utc);
                            let age_in_days = (now - created_at_utc).num_days();

                            // --- THE DATE SEARCH LOGIC ---
                            if age_in_days <= days_since_creation {
                                println!("🚩 {} WALLET DETECTED", "NEW".on_green().black());
                                println!("   Address:  {}", address.cyan());
                                println!("   Created:  {}", created_str.yellow());
                                println!("   Age:      {} days", age_in_days);
                                println!("------------------------------------------------------");
                                found_count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("\n✅ Found {} wallets created in the last {} days.", found_count, days_since_creation);
    Ok(())
}