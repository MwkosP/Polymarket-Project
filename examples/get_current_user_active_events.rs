use polyfill_rs::Result;
use serde_json::Value;
use colored::*;
use tokio::time::{sleep, Duration};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    // --- SETTINGS ---
    let user_address = "0xdbade4c82fb72780a0db9a38f821d8671aba9c95"; 
    // ----------------

    let client = reqwest::Client::builder()
        .user_agent("WhaleScanner/2.0")
        .build()?;

    println!("\n════════════════════════════════════════════════════════════════════════════════════════");
    println!("       ACTIVE OPEN EVENTS | GROUPED VIEW");
    println!("       WALLET: {}", user_address.cyan());
    println!("════════════════════════════════════════════════════════════════════════════════════════\n");

    let mut event_map: HashMap<String, (String, f64, usize)> = HashMap::new();
    let mut offset = 0;
    let limit = 100;

    loop {
        let url = format!(
            "https://data-api.polymarket.com/positions?user={}&limit={}&offset={}", 
            user_address, limit, offset
        );

        let resp = client.get(&url).send().await?.json::<Value>().await?;
        
        if let Some(arr) = resp.as_array() {
            if arr.is_empty() { break; }
            for pos in arr {
                let display_name = pos["groupTitle"].as_str()
                    .or(pos["title"].as_str())
                    .unwrap_or("Unknown");
                
                let event_id = pos["eventId"].as_str()
                    .unwrap_or_else(|| pos["slug"].as_str().unwrap_or(display_name))
                    .to_string();

                let val = pos["currentValue"].as_f64().unwrap_or(0.0);
                
                // Grouping markets into Events
                let entry = event_map.entry(event_id).or_insert((display_name.to_string(), 0.0, 0));
                entry.1 += val; 
                entry.2 += 1;   
            }
            if arr.len() < limit { break; }
            offset += limit;
            sleep(Duration::from_millis(100)).await;
        } else { break; }
    }

    let mut sorted_events: Vec<_> = event_map.into_iter().collect();
    sorted_events.sort_by(|a, b| b.1.1.partial_cmp(&a.1.1).unwrap());

    println!("{:<5} | {:<55} | {:<8} | {}", "ID", "EVENT NAME", "MARKETS", "TOTAL EXPOSURE");
    println!("────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────");

    let mut total_portfolio_value = 0.0;
    for (idx, (_id, (name, val, count))) in sorted_events.iter().enumerate() {
        total_portfolio_value += val;
        println!(
            "{:<5} | {:<55.55} | {:<8} | ${:>15}",
            (idx + 1).to_string().dimmed(),
            name.cyan(),
            count.to_string().white(),
            format_with_commas(*val) // Call our new helper here
        );
    }

    println!("────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────");
    println!("📊 TOTAL UNIQUE EVENTS:   {}", sorted_events.len().to_string().yellow().bold());
    println!("💰 TOTAL OPEN EXPOSURE:   ${}", format_with_commas(total_portfolio_value).green().bold());
    println!("════════════════════════════════════════════════════════════════════════════════════════\n");

    Ok(())
}

/// Helper function to format numbers with commas and 2 decimal places (e.g., 1,234,567.89)
fn format_with_commas(val: f64) -> String {
    let s = format!("{:.2}", val);
    let parts: Vec<&str> = s.split('.').collect();
    let integer_part = parts[0];
    let fractional_part = parts[1];

    let mut result = String::new();
    let mut count = 0;
    
    // Iterate backwards through integer part to add commas
    for c in integer_part.chars().rev() {
        if count > 0 && count % 3 == 0 && c != '-' {
            result.push(',');
        }
        result.push(c);
        count += 1;
    }
    
    format!("{}.{}", result.chars().rev().collect::<String>(), fractional_part)
}