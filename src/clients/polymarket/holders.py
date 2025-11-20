import requests
from collections import defaultdict
import json


# ------------------------------------------------------------
# Fetch raw holders data from Polymarket /holders endpoint
# ------------------------------------------------------------
def _get_raw_holders(condition_id, limit=500, min_balance=1):
    url = "https://data-api.polymarket.com/holders"
    params = {
        "market": condition_id,
        "limit": limit,
        "minBalance": min_balance
    }

    resp = requests.get(url, params=params)
    resp.raise_for_status()
    return resp.json()



# ------------------------------------------------------------
# Merge duplicate holder entries (same wallet appearing twice)
# ------------------------------------------------------------
def _merge_holders(holder_list):
    merged = defaultdict(float)
    for h in holder_list:
        merged[h["proxyWallet"]] += float(h["amount"])
    return dict(merged)



# ------------------------------------------------------------
# Main function: fetch holders for a specific market
# ------------------------------------------------------------
def fetch_holders(market, messages=True):
    """
    market: full market JSON object
    messages=True → print detailed output
    messages=False → only print minimal info, return data silently

    Returns:
    {
        "YES": { wallet: balance, ... },
        "NO": { wallet: balance, ... }
    }
    """

    condition_id = market["conditionId"]
    outcomes = market["outcomes"]

    # outcomes might be a JSON string → parse it
    if isinstance(outcomes, str):
        outcomes = json.loads(outcomes)

    title = market["question"]

    # Minimal message mode
    if not messages:
        print(f"\n=== Fetching Holders for Market: {title} ===")
    else:
        print("\n==================================================")
        print(f"Fetching holders for: {title}")
        print(f"Condition ID: {condition_id}")
        print("==================================================")

    # Fetch data
    try:
        raw_data = _get_raw_holders(condition_id)
    except Exception as e:
        if messages:
            print("❌ Error fetching holders:", e)
        return None

    results = {}

    # Process each outcome
    for outcome_group in raw_data:

        token_id = outcome_group["token"]
        holder_entries = outcome_group["holders"]

        if not holder_entries:
            if messages:
                print(f"\nNo holders found for token {token_id}")
            continue

        # Determine outcome name
        outcome_index = holder_entries[0]["outcomeIndex"]
        outcome_name = outcomes[outcome_index]   # "Yes" or "No"

        # Merge duplicates
        merged = _merge_holders(holder_entries)

        # Store result in dictionary
        results[outcome_name.upper()] = merged

        # Verbose printing
        if messages:
            print(f"\nOutcome: {outcome_name.upper()}")
            print(f"Token ID: {token_id}")
            print(f"Unique holders: {len(merged)}")

            sorted_holders = sorted(merged.items(), key=lambda x: x[1], reverse=True)

            for wallet, amount in sorted_holders:
                print(f"- {wallet}: {amount}")

    if messages:
        print("\n==================================================\n")

    return results
