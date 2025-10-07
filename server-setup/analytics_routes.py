# Additional routes for tarot reading analytics
# Add these to routes.py in the tarot-api

from fastapi import APIRouter, Depends, HTTPException
from sqlalchemy.orm import Session
from sqlalchemy import func
from typing import List, Dict
from collections import Counter

from database import get_db
from models import User, Reading, CardDrawn, CardDatabase, ReadingAttribute
from schemas import ReadingResponse

analytics_router = APIRouter()

@analytics_router.get("/readings/user/{matrix_id}/history")
def get_user_history(matrix_id: str, db: Session = Depends(get_db)):
    """Get reading history for a user with card details"""
    user = db.query(User).filter(User.matrix_id == matrix_id).first()
    if not user:
        return {
            "total_readings": 0,
            "readings": []
        }
    
    readings = db.query(Reading)\
        .filter(Reading.user_id == user.user_id)\
        .order_by(Reading.reading_date.desc())\
        .all()
    
    history = []
    for reading in readings:
        cards = db.query(CardDrawn)\
            .filter(CardDrawn.reading_id == reading.reading_id)\
            .order_by(CardDrawn.position)\
            .all()
        
        history.append({
            "reading_id": reading.reading_id,
            "spread_type": reading.spread_type,
            "reading_date": reading.reading_date.isoformat(),
            "card_count": len(cards),
            "cards": [{"position": c.position, "card_name": c.card_name, "label": c.card_label} for c in cards],
            "notes": reading.notes
        })
    
    return {
        "total_readings": len(readings),
        "readings": history
    }

@analytics_router.get("/readings/{reading_id}/details")
def get_reading_details(reading_id: int, db: Session = Depends(get_db)):
    """Get detailed information about a specific reading"""
    reading = db.query(Reading).filter(Reading.reading_id == reading_id).first()
    if not reading:
        raise HTTPException(status_code=404, detail="Reading not found")
    
    cards = db.query(CardDrawn)\
        .filter(CardDrawn.reading_id == reading_id)\
        .order_by(CardDrawn.position)\
        .all()
    
    # Get card details from database
    card_details = []
    for card_drawn in cards:
        card_info = db.query(CardDatabase)\
            .filter(CardDatabase.card_name == card_drawn.card_name)\
            .first()
        
        card_details.append({
            "position": card_drawn.position,
            "card_name": card_drawn.card_name,
            "label": card_drawn.card_label,
            "info": card_info.info if card_info and card_info.info else None,
            "deepinfo": card_info.deepinfo if card_info and card_info.deepinfo else None
        })
    
    # Get attribute counts
    attributes = db.query(ReadingAttribute)\
        .filter(ReadingAttribute.reading_id == reading_id)\
        .all()
    
    attribute_counts = {}
    for attr in attributes:
        if attr.attribute_type not in attribute_counts:
            attribute_counts[attr.attribute_type] = {}
        attribute_counts[attr.attribute_type][attr.attribute_value] = attr.count
    
    return {
        "reading_id": reading.reading_id,
        "spread_type": reading.spread_type,
        "reading_date": reading.reading_date.isoformat(),
        "notes": reading.notes,
        "cards": card_details,
        "attributes": attribute_counts
    }

@analytics_router.get("/analytics/user/{matrix_id}/attributes/{attribute_type}")
def get_attribute_frequency(
    matrix_id: str,
    attribute_type: str,
    db: Session = Depends(get_db)
):
    """Get frequency distribution of a specific attribute type across all user readings"""
    user = db.query(User).filter(User.matrix_id == matrix_id).first()
    if not user:
        return {"attribute_type": attribute_type, "frequencies": {}}
    
    # Query aggregated attribute counts
    results = db.query(
        ReadingAttribute.attribute_value,
        func.sum(ReadingAttribute.count).label('total_count')
    ).join(Reading)\
     .filter(Reading.user_id == user.user_id)\
     .filter(ReadingAttribute.attribute_type == attribute_type)\
     .group_by(ReadingAttribute.attribute_value)\
     .order_by(func.sum(ReadingAttribute.count).desc())\
     .all()
    
    frequencies = {row.attribute_value: row.total_count for row in results}
    total = sum(frequencies.values())
    
    # Calculate percentages
    percentages = {k: (v / total * 100) if total > 0 else 0 for k, v in frequencies.items()}
    
    return {
        "attribute_type": attribute_type,
        "total_count": total,
        "frequencies": frequencies,
        "percentages": percentages
    }

@analytics_router.get("/analytics/user/{matrix_id}/summary")
def get_user_analytics_summary(matrix_id: str, db: Session = Depends(get_db)):
    """Get comprehensive analytics summary for a user"""
    user = db.query(User).filter(User.matrix_id == matrix_id).first()
    if not user:
        return {"error": "User not found"}
    
    # Total readings
    total_readings = db.query(Reading)\
        .filter(Reading.user_id == user.user_id)\
        .count()
    
    # Total cards drawn
    total_cards = db.query(func.count(CardDrawn.card_id))\
        .join(Reading)\
        .filter(Reading.user_id == user.user_id)\
        .scalar()
    
    # Most common spread type
    spread_counts = db.query(
        Reading.spread_type,
        func.count(Reading.reading_id).label('count')
    ).filter(Reading.user_id == user.user_id)\
     .group_by(Reading.spread_type)\
     .order_by(func.count(Reading.reading_id).desc())\
     .all()
    
    # Get top attributes for each type
    attribute_types = ['suit', 'element', 'planet', 'sign', 'sephira']
    top_attributes = {}
    
    for attr_type in attribute_types:
        top_3 = db.query(
            ReadingAttribute.attribute_value,
            func.sum(ReadingAttribute.count).label('total')
        ).join(Reading)\
         .filter(Reading.user_id == user.user_id)\
         .filter(ReadingAttribute.attribute_type == attr_type)\
         .group_by(ReadingAttribute.attribute_value)\
         .order_by(func.sum(ReadingAttribute.count).desc())\
         .limit(3)\
         .all()
        
        top_attributes[attr_type] = [{"value": row.attribute_value, "count": row.total} for row in top_3]
    
    return {
        "total_readings": total_readings,
        "total_cards_drawn": total_cards,
        "spread_types": [{"type": row.spread_type, "count": row.count} for row in spread_counts],
        "top_attributes": top_attributes
    }


# Function to calculate and store attributes when saving a reading
def calculate_reading_attributes(reading_id: int, card_names: List[str], db: Session):
    """Calculate and store attribute counts for a reading"""
    from collections import Counter
    
    # Load card database
    cards_data = db.query(CardDatabase).filter(CardDatabase.card_name.in_(card_names)).all()
    
    # Count attributes
    planets = Counter()
    signs = Counter()
    sephira = Counter()
    suits = Counter()
    elements = Counter()
    
    for card in cards_data:
        # Count planets
        if card.planet_orb:
            planets[card.planet_orb] += 1
        if card.planet_house:
            planets[card.planet_house] += 1
        
        # Count signs
        for sign_field in [card.sign_1, card.sign_2, card.sign_3]:
            if sign_field:
                signs[sign_field] += 1
        
        # Count sephira
        if card.sephira:
            sephira[card.sephira] += 1
        
        # Count suits
        for suit_field in [card.suit_1, card.suit_2]:
            if suit_field:
                suits[suit_field] += 1
        
        # Count elements
        for element_field in [card.element_1, card.element_2]:
            if element_field:
                elements[element_field] += 1
    
    # Store in database
    attribute_data = [
        ('planet', planets),
        ('sign', signs),
        ('sephira', sephira),
        ('suit', suits),
        ('element', elements)
    ]
    
    for attr_type, counter in attribute_data:
        for value, count in counter.items():
            attr = ReadingAttribute(
                reading_id=reading_id,
                attribute_type=attr_type,
                attribute_value=value,
                count=count
            )
            db.add(attr)
    
    db.commit()
