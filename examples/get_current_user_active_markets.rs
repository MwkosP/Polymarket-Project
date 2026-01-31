use polyfill_rs::Result;
use serde_json::Value;
use colored::*;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    // --- SETTINGS ---
    let user_address = "0xdbade4c82fb72780a0db9a38f821d8671aba9c95"; 
    // ----------------

    let client = reqwest::Client::builder()
        .user_agent("PolymarketMarketScanner/1.0")
        .build()?;

    println!("\n════════════════════════════════════════════════════════════════════════════════════════");
    println!("       ACTIVE OPEN MARKETS | INDIVIDUAL CONTRACTS");
    println!("       WALLET: {}", user_address.cyan());
    println!("════════════════════════════════════════════════════════════════════════════════════════\n");

    let mut master_list = Vec::new();
    let mut offset = 0;
    let limit = 100;

    // Fetch only from the 'positions' endpoint (Active/Open only)
    loop {
        let url = format!(
            "https://data-api.polymarket.com/positions?user={}&limit={}&offset={}", 
            user_address, limit, offset
        );

        let resp = client.get(&url).send().await?;
        
        // Handle rate limiting
        if resp.status() == 429 {
            println!("{}", "⚠️ Rate limit hit. Waiting 5 seconds...".yellow());
            sleep(Duration::from_secs(5)).await;
            continue;
        }

        let list: Value = resp.json().await?;
        if let Some(arr) = list.as_array() {
            if arr.is_empty() { break; }
            
            // Filter: Only add markets where the user still holds shares
            for pos in arr {
                let shares = pos["size"].as_f64().unwrap_or(0.0);
                if shares > 0.001 {
                    master_list.push(pos.clone());
                }
            }

            if arr.len() < limit { break; }
            offset += limit;
            
            // Polite pause between pages
            sleep(Duration::from_millis(150)).await;
        } else {
            break;
        }
    }

    // --- DISPLAY TABLE ---
    println!("{:<5} | {:<45} | {:<8} | {:<12} | {}", 
        "ID", "MARKET NAME", "SIDE", "SHARES", "CURRENT VALUE"
    );
    println!("────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────");

    let mut total_active_value = 0.0;

    for (idx, pos) in master_list.iter().enumerate() {
        let title = pos["title"].as_str().unwrap_or("Unknown Market");
        let outcome = pos["outcome"].as_str().unwrap_or("--");
        
        // size = total shares held
        let shares = pos["size"].as_f64().unwrap_or(0.0);
        
        // currentValue = total dollar value based on latest market price
        let val = pos["currentValue"].as_f64().unwrap_or(0.0);
        
        total_active_value += val;

        println!(
            "{:<5} | {:<45.45} | {:<8} | {:<12.1} | ${:<12.2}",
            (idx + 1).to_string().dimmed(),
            title.cyan(),
            if outcome == "Yes" { "YES".magenta() } else { "NO".blue() },
            shares,
            val
        );
    }

    println!("────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────");
    println!("📊 TOTAL ACTIVE MARKETS FOUND: {}", master_list.len().to_string().yellow().bold());
    println!("💰 TOTAL OPEN EXPOSURE:        ${:.2}", total_active_value.to_string().green().bold());
    println!("════════════════════════════════════════════════════════════════════════════════════════\n");

    Ok(())
}