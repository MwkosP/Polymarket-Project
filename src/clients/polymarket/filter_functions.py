#---------------------------------
# Filter by Keyword
#---------------------------------

def filter_by_keywords(
    events,
    markets,
    keywords,
    field_events="title",
    field_markets="question",
    messages=False,
    print_events=False,
    print_markets=False
):
    """
    Filter events + markets by keyword(s).
    messages=False → only prints one high-level line.
    messages=True  → prints full detailed results.
    """

    # Normalize keywords
    if isinstance(keywords, str):
        keywords = [keywords]

    if not messages:
        print(f"\n=== Filtering by keywords... ===")

    keywords_lower = [k.lower() for k in keywords]

    def match(text):
        if not text:
            return False
        t = text.lower()
        return any(k in t for k in keywords_lower)

    # Filter events
    filtered_events = []
    for e in events:
        text = f"{e.get(field_events, '')} {e.get('slug', '')}"
        if match(text):
            filtered_events.append(e)

    # Filter markets
    filtered_markets = []
    for m in markets:
        text = f"{m.get(field_markets, '')} {m.get('slug', '')}"
        if match(text):
            filtered_markets.append(m)

    # Verbose printing
    if messages:
        print(f"\n=== Matching events for {keywords} ===\n")
        for i, e in enumerate(filtered_events, 1):
            slug = e.get("slug", "")
            print(f"{i}. {e.get('title','')} → https://polymarket.com/event/{slug}")

        print(f"\nTotal Events: {len(filtered_events)}")

        if print_markets:
            print(f"\n=== Matching markets ===\n")
            for i, m in enumerate(filtered_markets, 1):
                slug = m.get("slug", "")
                print(f"{i}. {m.get('question','')} → https://polymarket.com/market/{slug}")

    return filtered_events, filtered_markets



#---------------------------------
# Filter by Expiration
#---------------------------------

from datetime import datetime, timedelta, timezone

def extract_event_expiration(event):
    keys = ["endTime", "endDate", "closedTime", "startTime"]

    for key in keys:
        val = event.get(key)
        if not val:
            continue

        try:
            return datetime.fromisoformat(val.replace("Z", "+00:00"))
        except:
            pass

    return None


def filter_by_expiration(events, max_days=10, messages=False):
    """
    Returns events expiring in >= max_days.
    messages=False → prints one high-level line.
    messages=True  → prints all matched events.
    """

    if not messages:
        print(f"\n=== Filtering events by expiration (≥ {max_days} days) ===")

    now = datetime.now(timezone.utc)
    limit = now + timedelta(days=max_days)

    result = []

    for e in events:
        end_dt = extract_event_expiration(e)
        if end_dt and end_dt >= limit:
            result.append(e)

    if messages:
        print(f"\n=== Filtering by Expiration ===\n")
        for i, e in enumerate(result, 1):
            slug = e.get("slug", "")
            end_dt = extract_event_expiration(e)
            exp_str = end_dt.strftime("%Y-%m-%d %H:%M UTC") if end_dt else "Unknown"
            print(f"{i}. {e.get('title','')} (Expires {exp_str}) → https://polymarket.com/event/{slug}")

        print(f"\nTotal Events: {len(result)}\n")

    return result
