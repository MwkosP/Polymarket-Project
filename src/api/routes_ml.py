from fastapi import APIRouter

router = APIRouter(prefix="/ml", tags=["3. ML"])

@router.get("/")
def utils_root():
    return {"message": "🛠 ML API online"}