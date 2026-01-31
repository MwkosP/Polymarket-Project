use polyfill_rs::Result;
use serde_json::Value;
use colored::*;
use chrono::{Utc, DateTime};

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::builder()
        .user_agent("WhaleScanner/2.0")
        .build()?;

    // --- VARIABLES ---
    let user_address = "0x2efa5262cab2b2a2f6e265765c05481a6f9cb8a9"; 
    // -----------------

    println!("\n══════════════════════════════════════════════════════");
    println!("       ACCOUNT GENESIS");
    println!("       WALLET: {}", user_address.cyan());
    println!("══════════════════════════════════════════════════════\n");

    // The Gamma API is the correct source for Profile Metadata
    let url = format!("https://gamma-api.polymarket.com/public-profile?address={}", user_address);
    
    let resp = client.get(&url).send().await?;
    
    // Check if the request was successful
    if !resp.status().is_success() {
        println!("{} Could not find profile. The user might not have a public profile yet.", "ERROR:".red());
        return Ok(());
    }

    let profile: Value = resp.json().await?;

    // Polymarket Gamma API usually stores this in 'created_at' or 'createdAt'
    if let Some(created_at_str) = profile["createdAt"].as_str() {
        let joined_date = created_at_str.parse::<DateTime<Utc>>()
            .expect("Failed to parse date string");
        
        let now = Utc::now();
        let days_old = now.signed_duration_since(joined_date).num_days();

        println!("📅 Joined Date:  {}", joined_date.format("%B %Y").to_string().yellow().bold());
        println!("🕒 Exact Time:   {}", joined_date.format("%Y-%m-%d %H:%M").to_string().dimmed());
        println!("⏳ Total Age:    {} days", days_old.to_string().bold());

        if days_old < 60 {
            println!("\n🚨 {} This account is very new.", "FRESH WHALE:".on_red());
        } else {
            println!("\n✅ {} This is an established account.", "MATURE:".on_green());
        }
    } else {
        println!("{}", "⚠️ Account creation date is hidden or not set for this profile.".yellow());
    }

    println!("\n══════════════════════════════════════════════════════\n");

    Ok(())
}