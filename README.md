# 🔮 Polymarket Market Scanner & Holder Analyzer

A Python toolkit for discovering Polymarket events, filtering markets, and fetching the top on-chain holders for any outcome (YES/NO).  
Built with clarity and automation in mind — no manual condition IDs, no guessing.

---

## 🚀 Features

- Fetch Polymarket events & markets by category  
- Filter markets by keywords  
- Filter events by expiration window  
- Automatically extract `conditionId` from any selected market  
- Fetch **top on-chain holders** for each outcome using Polymarket’s `/holders` API  
- Automatically merge duplicate holder entries  
- Clean output for both verbose and silent modes  
- Returns structured holder data for downstream analytics

---

## 📂 Project Structure
project/
│
├── data/ # saved JSON / outputs (optional)
├── src/
│ ├── clients/
│ │ └── polymarket/
│ │ ├── fetch_events.py
│ │ ├── filter_functions.py
│ │ ├── holders.py
│ │ └── constants.py
│ └── utils/ # shared tools (optional)
│
├── main.py
├── requirements.txt
├── .env
├── README.md
└── .gitignore



---









