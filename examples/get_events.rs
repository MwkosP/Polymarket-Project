use polyfill_rs::Result;
use serde_json::Value;
use colored::*; 

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let client = reqwest::Client::new();
    
    let mut target_events = Vec::new(); 
    let mut page = 0;
    let limit = 100; 
    let max_display = 100; 


    let tag_id = 21; 
    // 1. Array of terms to look for
    let query_terms = ["bitcoin", "btc"]; 
    // We'll use the first term for the API request query parameter
    let api_query = query_terms[0]; 

    println!("\n══════════════════════════════════════════════════════");
    println!("       SCANNING FOR: {}", query_terms.join(" / ").to_uppercase().bold().yellow());
    println!("══════════════════════════════════════════════════════\n");

    loop {
        let offset = page * limit;
        let url = format!(
            "https://gamma-api.polymarket.com/events?active=true&closed=false&tag_id={}&query={}&limit={}&offset={}&order=volume&dir=desc",
            tag_id, api_query, limit, offset
        );

        let resp = client.get(&url).send().await.expect("Network Error");
        let payload: Value = resp.json().await.expect("JSON Error");

        if let Some(event_list) = payload.as_array() {
            if event_list.is_empty() { break; }
            for e in event_list {
                let title = e["title"].as_str().unwrap_or("").to_lowercase();
                
                // 2. CHECK IF TITLE CONTAINS ANY OF YOUR TERMS
                if query_terms.iter().any(|&term| title.contains(term)) {
                    target_events.push(e.clone());
                }
            }
            if target_events.len() >= max_display { break; }
            page += 1;
        } else { break; }
    }

    println!("\n✅ Found {} matching events. Showing top {}.", target_events.len(), max_display);
    println!("──────────────────────────────────────────────────────\n");

    for (i, e) in target_events.iter().take(max_display).enumerate() {
        let title = e["title"].as_str().unwrap_or("No Title");
        let slug = e["slug"].as_str().unwrap_or("");
        let total_vol = e["volume"].as_str().unwrap_or("0");
        let liquidity = e["liquidity"].as_str().unwrap_or("0");
        
        let mut live_output = Vec::new();

        if let Some(markets) = e["markets"].as_array() {
            for m in markets {
                let label = m["groupItemTitle"].as_str().or(m["question"].as_str()).unwrap_or("Outcome");
                //For getting Users Data 
                let condition_id = m["conditionId"].as_str().unwrap_or("N/A");

                // --- HIDDEN CALCULATIONS START ---
                let prices_raw = m["outcomePrices"].as_str().unwrap_or("[]");
                let prices: Value = serde_json::from_str(prices_raw).unwrap_or(Value::Array(vec![]));
                
                let yes_price_str = prices.get(0).and_then(|v| v.as_str()).unwrap_or("0.00");
                let yes_price_f64 = yes_price_str.parse::<f64>().unwrap_or(0.0);
                
                let yes_pct = (yes_price_f64 * 100.0).round();
                let no_pct = (100.0 - yes_pct).round();

                let tokens_raw = m["clobTokenIds"].as_str().unwrap_or("[]");
                let tokens: Value = serde_json::from_str(tokens_raw).unwrap_or(Value::Array(vec![]));
                let _yes_token_id = tokens.get(0).and_then(|v| v.as_str()).unwrap_or("N/A");
                let _no_token_id = tokens.get(1).and_then(|v| v.as_str()).unwrap_or("N/A");
                // --- HIDDEN CALCULATIONS END ---

                let yes_display = yes_pct.to_string().purple();
                let no_display = no_pct.to_string().purple();

                live_output.push(format!(
                    "      → {:<20} | Price: ${:<4} | YES: {}% | NO: {}% | {}", 
                    label, yes_price_str, yes_display, no_display,condition_id
                ));
            }
        }

        if !live_output.is_empty() {
            println!("[{:2}] {} [ Vol: ${} | Liq: ${} ]", (i + 1).to_string().white(), title.purple(), total_vol.cyan(), liquidity.cyan());
            for line in live_output {
                println!("{}", line);
            }
            println!("      🔗 LINK: https://polymarket.com/event/{}", slug);
            println!("      ──────────────────────────────────────────────────────");
        }
    }

    Ok(())
}