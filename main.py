

from src.clients.polymarket.fetch_events import *
from src.clients.polymarket.filter_functions import *
from src.clients.polymarket.constants import *
from src.clients.polymarket.holders import *
import json




def main():
    print("Polymarket Program running...")

    events, markets = fetch_category(TOPICS[6]["category"])
    #events, markets = filter_by_keywords(events, markets, TOPICS[6]["keywords"])
    #events = filter_by_expiration(events, max_days=30, messages=True)

    # automatically select the first market returned
    if not markets:
        print("No markets found.")
        return

    market = markets[90]   # <-- automatic
    print(market)
    fetch_holders(market)   # prints full details


if __name__ == "__main__":
    main()
