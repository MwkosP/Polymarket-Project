import requests

BASE = "https://gamma-api.polymarket.com"


# ----------------------------------------------------
# 1) FETCH ALL API ITEMS FOR A GIVEN ENDPOINT + TAG ID
# ----------------------------------------------------
def fetch_all(endpoint, tag_id, messages=False):
    items = []
    limit = 100
    offset = 0

    while True:
        url = f"{BASE}/{endpoint}?tag_id={tag_id}&closed=false&limit={limit}&offset={offset}"
        data = requests.get(url, timeout=5).json()

        if not isinstance(data, list) or len(data) == 0:
            break

        if messages:
            print(f"[{endpoint}] fetched {len(data)} items at offset {offset}")

        items.extend(data)
        offset += limit

    return items


# ----------------------------------------------------
# 2) FETCH EVENTS + MARKETS FOR A SINGLE TAG
# ----------------------------------------------------
def fetch_polymarket_by_tag(tag_id, messages=False):
    if messages:
        print(f"\n=== Fetching tag {tag_id} ===")

    events = fetch_all("events", tag_id, messages=messages)
    markets = fetch_all("markets", tag_id, messages=messages)

    if messages:
        print("\n=================================================")
        print(f"Total events: {len(events)}")
        print(f"Total markets: {len(markets)}")

        print("\n=== EVENTS ===")
        for e in events:
            slug = e.get("slug") or e.get("event_slug") or ""
            print(f"- {e['id']}: {e['title']} → https://polymarket.com/event/{slug}")

        # MARKETS (disabled in original, kept optional)
        # print("\n=== MARKETS ===")
        # for m in markets:
        #     slug = m.get("slug") or m.get("market_slug") or ""
        #     print(f"- {m['question']} → https://polymarket.com/market/{slug}")

    return events, markets


# ----------------------------------------------------
# 3) TAG LISTS
# ----------------------------------------------------
CRYPTO = [21, 1312, 235, 39, 100171, 100170, 102134, 102536]
ECONOMY_FINANCE = [100196, 101550, 159, 100328, 101800, 107, 120]
TECH_AI = [439, 101999, 1401, 102800, 540, 101734, 102022, 102846]
MOVIES_CULTURE = [51, 53, 1164, 596]
SPACE_SCIENCE = [63, 74]
POLITICS = [2, 101588, 198, 101970, 101794, 102505]
GEOPOLITICS = [270, 96, 192, 100265, 95, 102477, 102498, 154, 180, 78, 102304, 102083]
DRUGS = [100239, 102481, 102480, 102482, 101060]
MISC = [100215]

CATEGORIES = {
    "CRYPTO": CRYPTO,
    "ECONOMY_FINANCE": ECONOMY_FINANCE,
    "TECH_AI": TECH_AI,
    "MOVIES_CULTURE": MOVIES_CULTURE,
    "SPACE_SCIENCE": SPACE_SCIENCE,
    "POLITICS": POLITICS,
    "GEOPOLITICS": GEOPOLITICS,
    "DRUGS": DRUGS,
    "MISC": MISC,
}


# ----------------------------------------------------
# 4) FETCH ALL TAGS IN A CATEGORY
# ----------------------------------------------------
def fetch_category(category_name, messages=False):
    if category_name not in CATEGORIES:
        raise ValueError(f"Unknown category: {category_name}")

    tag_ids = CATEGORIES[category_name]
    all_events = []
    all_markets = []

    # HIGH-LEVEL PROGRESS MESSAGE
    print(f"\n===== Searching category '{category_name}'... =====")


    for tag_id in tag_ids:
        if messages:
            print(f"\n--- Tag {tag_id} ---")

        events, markets = fetch_polymarket_by_tag(tag_id, messages=messages)
        all_events.extend(events)
        all_markets.extend(markets)



    return all_events, all_markets
