use polyfill_rs::Result;
use serde_json::Value;
use colored::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::builder()
        .user_agent("WhaleScanner/2.0")
        .build()?;

    // --- SETTINGS ---
    let user_address = "0x9d84ce0306f8551e02efef1680475fc0f1dc1344"; 
    // ----------------

    println!("\n══════════════════════════════════════════════════════");
    println!("       USER USERNAME DECODER");
    println!("       WALLET: {}", user_address.cyan());
    println!("══════════════════════════════════════════════════════\n");

    let url = format!("https://gamma-api.polymarket.com/public-profile?address={}", user_address);
    let resp = client.get(&url).send().await?;

    if resp.status().is_success() {
        let profile: Value = resp.json().await?;

        // name is the user-chosen display name
        let display_name = profile["name"].as_str().unwrap_or("None Set");
        


        println!("👤 Username:     {}", display_name.green().bold());
   

        if display_name == "None Set" {
            println!("\nℹ️  {} This user is using their default address (anonymous).", "NOTICE:".dimmed());
        }

    } else {
        println!("{} Could not find a profile for this address.", "ERROR:".red());
    }

    println!("\n══════════════════════════════════════════════════════\n");

    Ok(())
}