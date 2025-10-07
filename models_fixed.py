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
    title = Column(String(100))
    image = Column(String(255))
    planet_orb = Column(String(50))
    planet_house = Column(String(50))
    sign_1 = Column(String(50))
    sign_2 = Column(String(50))
    sign_3 = Column(String(50))
    element_1 = Column(String(50))
    element_2 = Column(String(50))
    suit_1 = Column(String(50))
    suit_2 = Column(String(50))
    path = Column(String(50))
    sephira = Column(String(50))
    info = Column(Text)
    deepinfo = Column(Text)

class SpreadTemplate(Base):
    __tablename__ = "spread_templates"
    
    template_id = Column(Integer, primary_key=True, index=True)
    template_name = Column(String(100), unique=True, nullable=False)
    card_count = Column(Integer, nullable=False)
    positions = Column(JSONB, nullable=False)
    description = Column(Text)
    is_public = Column(Boolean, default=True)
    created_by = Column(Integer, ForeignKey("users.user_id"))

class ReadingAttribute(Base):
    __tablename__ = "reading_attributes"
    
    attribute_id = Column(Integer, primary_key=True, index=True)
    reading_id = Column(Integer, ForeignKey("readings.reading_id"), nullable=False)
    attribute_type = Column(String(50), nullable=False)
    attribute_value = Column(String(100), nullable=False)
    count = Column(Integer, nullable=False, default=1)
