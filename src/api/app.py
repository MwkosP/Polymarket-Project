from fastapi import FastAPI

from src.api import routes_data,routes_backtesting,routes_functions,routes_ml,routes_strategies,routes_utils




app = FastAPI(title="TA Backend", docs_url="/docs", redoc_url="/redoc")

@app.get("/", tags = ["ROOT"])
def root():
    return {"message": "✅ TA FastAPI backend running"}

# Include routers
app.include_router(routes_data.router)
app.include_router(routes_functions.router)
app.include_router(routes_ml.router)
app.include_router(routes_strategies.router)
app.include_router(routes_backtesting.router)
app.include_router(routes_utils.router)
