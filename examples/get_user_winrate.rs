use polyfill_rs::Result;
use serde_json::Value;
use colored::*;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::builder()
        .user_agent("WhaleScanner/3.0")
        .build()?;

    let user_address = "0x28065f1b88027422274fb33e1e22bf3dad5736e7"; 

    println!("\n══════════════════════════════════════════════════════");
    println!("       LIFETIME MARKET WIN-RATE (FULL SCAN)");
    println!("       WALLET: {}", user_address.cyan());
    println!("══════════════════════════════════════════════════════\n");

    let mut wins = 0;
    let mut losses = 0;
    let mut offset = 0;
    let limit = 50; 

    print!("🔍 Scanning all markets...");
    loop {
        let url = format!(
            "https://data-api.polymarket.com/closed-positions?user={}&limit={}&offset={}", 
            user_address, limit, offset
        );

        let resp = client.get(&url).send().await?.json::<Value>().await?;
        
        if let Some(closed_list) = resp.as_array() {
            if closed_list.is_empty() { break; }

            for pos in closed_list {
                let pnl = pos["realizedPnl"].as_f64().unwrap_or(0.0);
                if pnl > 0.01 { wins += 1; }
                else if pnl < -0.01 { losses += 1; }
            }

            if closed_list.len() < limit { break; }
            offset += limit;
            print!("."); 
            sleep(Duration::from_millis(100)).await;
        } else { break; }
    }
    println!(" Done!");

    // --- THE FIXED MATH ---
    let total_decided = wins + losses;
    
    // We calculate the raw float first
    let win_rate: f64 = if total_decided > 0 {
        (wins as f64 / total_decided as f64) * 100.0
    } else {
        0.0
    };

    println!("\n✅ Total Wins:       {}", wins.to_string().green().bold());
    println!("❌ Total Losses:     {}", losses.to_string().red().bold());
    println!("──────────────────────────────────────────────────────");
    
    // FIX: We apply the {:.2} format to the FLOAT directly, not a string
    println!("📊 CAREER WIN RATE:  {:.3}%", win_rate.to_string().yellow().bold());
    println!("📈 Total Decided:    {}", total_decided);
    println!("══════════════════════════════════════════════════════\n");

    Ok(())
}