from fastapi import APIRouter, Query
import pandas as pd
from src.data.fetch_yfinance import download_underlying_stock
from src.functions.indicators import INDICATOR_MAP

router = APIRouter(prefix="/functions",tags=["2. FUNCTIONS"]
)

# Root endpoint for FUNCTIONS
@router.get("/")
def root():
    return {"message": "✅ Functions API running"}



@router.get("/indicator")
def get_indicator(
    symbol: str = Query(..., example="BTC-USD"),
    start: str = Query("2023-09-01"),
    end: str = Query("2023-12-31"),
    interval: str = Query("1d"),
    indicator: str = Query(..., enum=list(INDICATOR_MAP.keys())),
    period: int = Query(14)
):
    df = download_underlying_stock(symbol, start, end, interval, plot=False)
    func = INDICATOR_MAP[indicator]
    result = func(df) if indicator not in ["ma", "rsi", "atr"] else func(df, period)

    # ✅ Let FastAPI auto-convert to JSON
    return result
