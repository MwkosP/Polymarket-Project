from fastapi import APIRouter

router = APIRouter(prefix="/strategies", tags=["4. STRATEGIES"])

@router.get("/")
def utils_root():
    return {"message": "🛠 Strategies API online"}