from fastapi import APIRouter

router = APIRouter(prefix="/backtesting", tags=["5. BACKTESTING"])

@router.get("/")
def utils_root():
    return {"message": "🛠 Backtesting API online"}