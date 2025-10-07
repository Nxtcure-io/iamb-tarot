#!/bin/bash

# Setup script for Tarot API service
# Run this on the AWS Lightsail server after running 01-setup-database.sh

set -e

echo "=========================================="
echo "Tarot API Service Setup"
echo "=========================================="
echo ""

API_DIR="$HOME/matrix/tarot-api"

echo "Creating API directory structure..."
mkdir -p "$API_DIR"
cd "$API_DIR"

# Create requirements.txt
cat > requirements.txt << 'EOF'
fastapi==0.104.1
uvicorn[standard]==0.24.0
sqlalchemy==2.0.23
psycopg2-binary==2.9.9
pydantic==2.5.0
python-dotenv==1.0.0
httpx==0.25.1
python-jose[cryptography]==3.3.0
passlib[bcrypt]==1.7.4
python-multipart==0.0.6
EOF

echo "✓ requirements.txt created"

# Create Dockerfile
cat > Dockerfile << 'EOF'
FROM python:3.11-slim

WORKDIR /app

# Install dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy application
COPY . .

# Expose port
EXPOSE 8080

# Run the application
CMD ["uvicorn", "app:app", "--host", "0.0.0.0", "--port", "8080"]
EOF

echo "✓ Dockerfile created"

# Create main application file
cat > app.py << 'EOF'
from fastapi import FastAPI, HTTPException, Depends
from fastapi.middleware.cors import CORSMiddleware
from sqlalchemy.orm import Session
import os
from dotenv import load_dotenv

from database import engine, SessionLocal, Base
from routes import router as api_router

# Load environment variables
load_dotenv()

# Create database tables
Base.metadata.create_all(bind=engine)

# Initialize FastAPI app
app = FastAPI(
    title="Tarot Readings API",
    description="API for managing tarot readings and Matrix user registration",
    version="1.0.0"
)

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=os.getenv("CORS_ORIGINS", "*").split(","),
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Include API routes
app.include_router(api_router, prefix="/api")

@app.get("/")
async def root():
    return {
        "service": "Tarot Readings API",
        "version": "1.0.0",
        "status": "running"
    }

@app.get("/health")
async def health_check():
    return {"status": "healthy"}
EOF

echo "✓ app.py created"

# Create database configuration
cat > database.py << 'EOF'
from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker
import os

DATABASE_URL = os.getenv("DATABASE_URL", "postgresql://tarot_user:password@localhost:5432/tarot_readings")

engine = create_engine(DATABASE_URL)
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
Base = declarative_base()

def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()
EOF

echo "✓ database.py created"

# Create models
cat > models.py << 'EOF'
from sqlalchemy import Column, Integer, String, Text, Boolean, DateTime, ForeignKey, ARRAY
from sqlalchemy.orm import relationship
from sqlalchemy.dialects.postgresql import JSONB
from datetime import datetime
from database import Base

class User(Base):
    __tablename__ = "users"
    
    user_id = Column(Integer, primary_key=True, index=True)
    matrix_id = Column(String(255), unique=True, nullable=False, index=True)
    username = Column(String(100), nullable=False)
    created_at = Column(DateTime, default=datetime.utcnow)
    last_reading_at = Column(DateTime)
    
    readings = relationship("Reading", back_populates="user", cascade="all, delete-orphan")

class Reading(Base):
    __tablename__ = "readings"
    
    reading_id = Column(Integer, primary_key=True, index=True)
    user_id = Column(Integer, ForeignKey("users.user_id"), nullable=False)
    room_id = Column(String(255))
    spread_type = Column(String(50), nullable=False)
    reading_date = Column(DateTime, default=datetime.utcnow, index=True)
    notes = Column(Text)
    is_private = Column(Boolean, default=False)
    
    user = relationship("User", back_populates="readings")
    cards = relationship("CardDrawn", back_populates="reading", cascade="all, delete-orphan")

class CardDrawn(Base):
    __tablename__ = "cards_drawn"
    
    card_id = Column(Integer, primary_key=True, index=True)
    reading_id = Column(Integer, ForeignKey("readings.reading_id"), nullable=False)
    position = Column(Integer, nullable=False)
    card_name = Column(String(100), nullable=False)
    card_label = Column(String(100))
    is_reversed = Column(Boolean, default=False)
    interpretation = Column(Text)
    
    reading = relationship("Reading", back_populates="cards")

class CardDatabase(Base):
    __tablename__ = "card_database"
    
    card_db_id = Column(Integer, primary_key=True, index=True)
    card_name = Column(String(100), unique=True, nullable=False)
    card_number = Column(String(10))
    card_title = Column(String(100), nullable=False)
    arcana_type = Column(String(20), nullable=False)
    suit = Column(String(20))
    upright_meaning = Column(Text)
    reversed_meaning = Column(Text)
    keywords = Column(ARRAY(Text))
    image_path = Column(String(255))

class SpreadTemplate(Base):
    __tablename__ = "spread_templates"
    
    template_id = Column(Integer, primary_key=True, index=True)
    template_name = Column(String(100), unique=True, nullable=False)
    card_count = Column(Integer, nullable=False)
    positions = Column(JSONB, nullable=False)
    description = Column(Text)
    is_public = Column(Boolean, default=True)
    created_by = Column(Integer, ForeignKey("users.user_id"))
EOF

echo "✓ models.py created"

# Create Pydantic schemas
cat > schemas.py << 'EOF'
from pydantic import BaseModel, Field
from typing import List, Optional
from datetime import datetime

class CardDrawnCreate(BaseModel):
    position: int
    card_name: str
    card_label: Optional[str] = None
    is_reversed: bool = False
    interpretation: Optional[str] = None

class CardDrawnResponse(CardDrawnCreate):
    card_id: int
    reading_id: int
    
    class Config:
        from_attributes = True

class ReadingCreate(BaseModel):
    matrix_id: str
    room_id: Optional[str] = None
    spread_type: str
    cards: List[CardDrawnCreate]
    notes: Optional[str] = None
    is_private: bool = False

class ReadingResponse(BaseModel):
    reading_id: int
    user_id: int
    room_id: Optional[str]
    spread_type: str
    reading_date: datetime
    notes: Optional[str]
    is_private: bool
    cards: List[CardDrawnResponse]
    
    class Config:
        from_attributes = True

class UserCreate(BaseModel):
    matrix_id: str
    username: str

class UserResponse(BaseModel):
    user_id: int
    matrix_id: str
    username: str
    created_at: datetime
    last_reading_at: Optional[datetime]
    
    class Config:
        from_attributes = True

class MatrixRegisterRequest(BaseModel):
    username: str
    password: str
    admin: bool = False

class MatrixRegisterResponse(BaseModel):
    matrix_id: str
    username: str
    success: bool
    message: str

class CardInfo(BaseModel):
    card_name: str
    card_title: str
    card_number: str
    arcana_type: str
    upright_meaning: str
    reversed_meaning: str
    keywords: List[str]
    
    class Config:
        from_attributes = True
EOF

echo "✓ schemas.py created"

# Create API routes
cat > routes.py << 'EOF'
from fastapi import APIRouter, Depends, HTTPException, status
from sqlalchemy.orm import Session
from typing import List
import httpx
import os

from database import get_db
from models import User, Reading, CardDrawn, CardDatabase, SpreadTemplate
from schemas import (
    ReadingCreate, ReadingResponse, UserCreate, UserResponse,
    MatrixRegisterRequest, MatrixRegisterResponse, CardInfo
)

router = APIRouter()

MATRIX_SERVER = os.getenv("MATRIX_SERVER", "https://endlessperfect.com")

# User endpoints
@router.post("/users", response_model=UserResponse)
def create_user(user: UserCreate, db: Session = Depends(get_db)):
    """Create a new user"""
    db_user = db.query(User).filter(User.matrix_id == user.matrix_id).first()
    if db_user:
        raise HTTPException(status_code=400, detail="User already exists")
    
    new_user = User(**user.dict())
    db.add(new_user)
    db.commit()
    db.refresh(new_user)
    return new_user

@router.get("/users/{matrix_id}", response_model=UserResponse)
def get_user(matrix_id: str, db: Session = Depends(get_db)):
    """Get user by Matrix ID"""
    user = db.query(User).filter(User.matrix_id == matrix_id).first()
    if not user:
        raise HTTPException(status_code=404, detail="User not found")
    return user

# Reading endpoints
@router.post("/readings", response_model=ReadingResponse)
def create_reading(reading: ReadingCreate, db: Session = Depends(get_db)):
    """Save a new tarot reading"""
    # Get or create user
    user = db.query(User).filter(User.matrix_id == reading.matrix_id).first()
    if not user:
        user = User(matrix_id=reading.matrix_id, username=reading.matrix_id.split(':')[0][1:])
        db.add(user)
        db.commit()
        db.refresh(user)
    
    # Create reading
    new_reading = Reading(
        user_id=user.user_id,
        room_id=reading.room_id,
        spread_type=reading.spread_type,
        notes=reading.notes,
        is_private=reading.is_private
    )
    db.add(new_reading)
    db.commit()
    db.refresh(new_reading)
    
    # Add cards
    for card_data in reading.cards:
        card = CardDrawn(
            reading_id=new_reading.reading_id,
            **card_data.dict()
        )
        db.add(card)
    
    db.commit()
    db.refresh(new_reading)
    
    # Update user's last reading time
    user.last_reading_at = new_reading.reading_date
    db.commit()
    
    return new_reading

@router.get("/readings/user/{matrix_id}", response_model=List[ReadingResponse])
def get_user_readings(matrix_id: str, limit: int = 50, db: Session = Depends(get_db)):
    """Get all readings for a user"""
    user = db.query(User).filter(User.matrix_id == matrix_id).first()
    if not user:
        return []
    
    readings = db.query(Reading)\
        .filter(Reading.user_id == user.user_id)\
        .order_by(Reading.reading_date.desc())\
        .limit(limit)\
        .all()
    
    return readings

@router.get("/readings/{reading_id}", response_model=ReadingResponse)
def get_reading(reading_id: int, db: Session = Depends(get_db)):
    """Get a specific reading"""
    reading = db.query(Reading).filter(Reading.reading_id == reading_id).first()
    if not reading:
        raise HTTPException(status_code=404, detail="Reading not found")
    return reading

@router.put("/readings/{reading_id}")
def update_reading(reading_id: int, notes: str, db: Session = Depends(get_db)):
    """Update reading notes"""
    reading = db.query(Reading).filter(Reading.reading_id == reading_id).first()
    if not reading:
        raise HTTPException(status_code=404, detail="Reading not found")
    
    reading.notes = notes
    db.commit()
    return {"success": True, "reading_id": reading_id}

@router.delete("/readings/{reading_id}")
def delete_reading(reading_id: int, db: Session = Depends(get_db)):
    """Delete a reading"""
    reading = db.query(Reading).filter(Reading.reading_id == reading_id).first()
    if not reading:
        raise HTTPException(status_code=404, detail="Reading not found")
    
    db.delete(reading)
    db.commit()
    return {"success": True, "reading_id": reading_id}

# Card database endpoints
@router.get("/cards", response_model=List[CardInfo])
def get_all_cards(db: Session = Depends(get_db)):
    """Get all cards from database"""
    cards = db.query(CardDatabase).all()
    return cards

@router.get("/cards/{card_name}", response_model=CardInfo)
def get_card(card_name: str, db: Session = Depends(get_db)):
    """Get specific card information"""
    card = db.query(CardDatabase).filter(CardDatabase.card_name == card_name).first()
    if not card:
        raise HTTPException(status_code=404, detail="Card not found")
    return card

# Spread templates
@router.get("/spreads")
def get_spreads(db: Session = Depends(get_db)):
    """Get all available spread templates"""
    spreads = db.query(SpreadTemplate).filter(SpreadTemplate.is_public == True).all()
    return spreads

# Matrix integration
@router.post("/matrix/register", response_model=MatrixRegisterResponse)
async def register_matrix_user(request: MatrixRegisterRequest):
    """Register a new user on the Matrix server"""
    try:
        # Call Matrix registration API
        async with httpx.AsyncClient() as client:
            response = await client.post(
                f"{MATRIX_SERVER}/_matrix/client/r0/register",
                json={
                    "username": request.username,
                    "password": request.password,
                    "admin": request.admin
                }
            )
            
            if response.status_code == 200:
                data = response.json()
                matrix_id = data.get("user_id")
                
                return MatrixRegisterResponse(
                    matrix_id=matrix_id,
                    username=request.username,
                    success=True,
                    message="User registered successfully"
                )
            else:
                raise HTTPException(
                    status_code=response.status_code,
                    detail=response.json().get("error", "Registration failed")
                )
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
EOF

echo "✓ routes.py created"

echo ""
echo "=========================================="
echo "API Service Files Created!"
echo "=========================================="
echo ""
echo "Next steps:"
echo "1. Build Docker image:"
echo "   cd $API_DIR"
echo "   docker build -t tarot-api ."
echo ""
echo "2. Update docker-compose.yml (run 03-update-docker-compose.sh)"
echo ""
echo "3. Start the service:"
echo "   docker compose up -d tarot-api"
echo ""
