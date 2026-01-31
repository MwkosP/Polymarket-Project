use polyfill_rs::Result;
use serde_json::Value;
use colored::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();
    let user_address = "0x2efa5262cab2b2a2f6e265765c05481a6f9cb8a9"; 

    // 1. Get the LIFETIME UNIQUE COUNT (Matches UI Profile)
    let traded_url = format!("https://data-api.polymarket.com/traded?user={}", user_address);
    let traded_resp = client.get(&traded_url).send().await?.json::<Value>().await?;
    let lifetime_total = traded_resp["traded"].as_u64().unwrap_or(0);

    // 2. Get the CURRENT OPEN COUNT
    let open_url = format!("https://data-api.polymarket.com/positions?user={}", user_address);
    let open_resp = client.get(&open_url).send().await?.json::<Value>().await?;
    let open_total = open_resp.as_array().map(|a| a.len()).unwrap_or(0);

    println!("\n══════════════════════════════════════════════════════");
    println!("       WHALE CAREER SUMMARY");
    println!("       WALLET: {}", user_address.cyan());
    println!("══════════════════════════════════════════════════════\n");

    println!("📊 TOTAL UNIQUE MARKETS (LIFETIME): {}", lifetime_total.to_string().yellow().bold());
    println!("🎯 CURRENTLY ACTIVE MARKETS:        {}", open_total.to_string().green().bold());
    println!("📉 CLOSED/HISTORICAL MARKETS:       {}", (lifetime_total - open_total as u64).to_string().dimmed());

    if lifetime_total > 500 {
        println!("\n🏆 Status: {}", "POLITICAL MASTERMIND (VETERAN)".magenta());
    } else {
        println!("\n🔍 Status: {}", "FOCUSED WHALE / SELECTIVE".blue());
    }

    println!("\n══════════════════════════════════════════════════════\n");

    Ok(())
}