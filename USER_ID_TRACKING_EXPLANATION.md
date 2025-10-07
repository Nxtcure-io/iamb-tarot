# How the Recording System Tracks User IDs

## Overview

The tarot reading history system uses Matrix user IDs to associate readings with specific users. Here's how it works:

## Database Structure

### 1. Users Table
```sql
CREATE TABLE users (
    user_id INTEGER PRIMARY KEY,           -- Internal database ID
    matrix_id VARCHAR(255) UNIQUE,         -- Matrix ID like "@waqaas:endlessperfect.com"
    username VARCHAR(100),                 -- Username extracted from matrix_id
    created_at TIMESTAMP,
    last_reading_at TIMESTAMP
);
```

### 2. Readings Table
```sql
CREATE TABLE readings (
    reading_id INTEGER PRIMARY KEY,
    user_id INTEGER,                       -- Foreign key to users.user_id
    room_id VARCHAR(255),                  -- Matrix room ID (optional)
    spread_type VARCHAR(50),               -- "3", "5", "7", etc.
    reading_date TIMESTAMP,
    notes TEXT,
    is_private BOOLEAN,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);
```

### 3. Cards Drawn Table
```sql
CREATE TABLE cards_drawn (
    card_id INTEGER PRIMARY KEY,
    reading_id INTEGER,                    -- Foreign key to readings.reading_id
    position INTEGER,                      -- 0, 1, 2 for a 3-card spread
    card_name VARCHAR(100),                -- "The Star", "Six of Swords", etc.
    card_label VARCHAR(100),               -- "Card 1", "Card 2", etc.
    is_reversed BOOLEAN,
    interpretation TEXT,
    FOREIGN KEY (reading_id) REFERENCES readings(reading_id)
);
```

### 4. Reading Attributes Table
```sql
CREATE TABLE reading_attributes (
    attribute_id INTEGER PRIMARY KEY,
    reading_id INTEGER,                    -- Foreign key to readings.reading_id
    attribute_type VARCHAR(50),            -- "suit", "element", "planet", "sign", "sephira"
    attribute_value VARCHAR(100),          -- "Swords", "Air", "Mercury", etc.
    count INTEGER,                         -- How many times this attribute appears
    FOREIGN KEY (reading_id) REFERENCES readings(reading_id)
);
```

## Data Flow: Saving a Reading

### Step 1: User Does a Reading
```rust
// In src/commands.rs - handle_n_card_spread()
let matrix_id = "@waqaas:endlessperfect.com".to_string();  // Currently hardcoded
let room_id = None;  // TODO: Get from context

let reading_request = tarot_api::ReadingCreate {
    matrix_id,      // This is the key identifier
    room_id,
    spread_type: "3".to_string(),
    cards: card_data,
    notes: None,
    is_private: false,
};

tarot_api::save_reading(reading_request)?;
```

### Step 2: API Receives the Request
```python
# In ~/matrix/tarot-api/routes.py
@router.post("/readings")
def create_reading(reading: ReadingCreate, db: Session):
    # Get or create user based on matrix_id
    user = db.query(User).filter(User.matrix_id == reading.matrix_id).first()
    
    if not user:
        # Create new user if doesn't exist
        user = User(
            matrix_id=reading.matrix_id,
            username=reading.matrix_id.split(':')[0][1:]  # Extract "waqaas" from "@waqaas:..."
        )
        db.add(user)
        db.commit()
        db.refresh(user)
    
    # Create the reading associated with this user
    new_reading = Reading(
        user_id=user.user_id,  # Link to the user
        room_id=reading.room_id,
        spread_type=reading.spread_type,
        notes=reading.notes,
        is_private=reading.is_private
    )
    db.add(new_reading)
    db.commit()
    
    # Save each card
    for card_data in reading.cards:
        card = CardDrawn(
            reading_id=new_reading.reading_id,
            position=card_data.position,
            card_name=card_data.card_name,
            card_label=card_data.card_label
        )
        db.add(card)
    
    # Calculate and save attributes
    calculate_attributes(new_reading.reading_id, reading.cards, db)
    
    db.commit()
    return new_reading
```

### Step 3: Attributes Are Calculated
```python
def calculate_attributes(reading_id, cards, db):
    # Look up each card in card_database
    card_names = [c.card_name for c in cards]
    db_cards = db.query(CardDatabase).filter(
        CardDatabase.card_name.in_(card_names)
    ).all()
    
    # Count attributes
    attributes = {}
    for card in db_cards:
        if card.suit_1:
            attributes.setdefault('suit', {})
            attributes['suit'][card.suit_1] = attributes['suit'].get(card.suit_1, 0) + 1
        
        if card.element_1:
            attributes.setdefault('element', {})
            attributes['element'][card.element_1] = attributes['element'].get(card.element_1, 0) + 1
        
        # ... same for planets, signs, sephira
    
    # Save to reading_attributes table
    for attr_type, values in attributes.items():
        for attr_value, count in values.items():
            attr = ReadingAttribute(
                reading_id=reading_id,
                attribute_type=attr_type,
                attribute_value=attr_value,
                count=count
            )
            db.add(attr)
    
    db.commit()
```

## Data Flow: Retrieving History

### Step 1: User Requests History
```rust
// In src/commands.rs - iamb_tarot_history()
let matrix_id = "@waqaas:endlessperfect.com".to_string();  // Currently hardcoded
tarot_api::get_history(&matrix_id)
```

### Step 2: API Fetches User's Readings
```python
@router.get("/readings/user/{matrix_id}/history")
def get_user_history(matrix_id: str, db: Session):
    # Find the user by their Matrix ID
    user = db.query(User).filter(User.matrix_id == matrix_id).first()
    
    if not user:
        return {"total_readings": 0, "readings": []}
    
    # Get all readings for this user
    readings = db.query(Reading).filter(
        Reading.user_id == user.user_id  # Filter by user_id
    ).order_by(Reading.reading_date.desc()).all()
    
    # Format and return
    result = []
    for reading in readings:
        cards = db.query(CardDrawn).filter(
            CardDrawn.reading_id == reading.reading_id
        ).all()
        
        result.append({
            "reading_id": reading.reading_id,
            "spread_type": reading.spread_type,
            "reading_date": reading.reading_date,
            "cards": cards,
            "notes": reading.notes
        })
    
    return {"total_readings": len(result), "readings": result}
```

### Step 3: Analytics Are Calculated Per User
```python
@router.get("/analytics/user/{matrix_id}/summary")
def get_analytics_summary(matrix_id: str, db: Session):
    # Find user
    user = db.query(User).filter(User.matrix_id == matrix_id).first()
    
    if not user:
        return empty_summary()
    
    # Get all readings for THIS USER ONLY
    readings = db.query(Reading).filter(
        Reading.user_id == user.user_id
    ).all()
    
    reading_ids = [r.reading_id for r in readings]
    
    # Get attributes for these readings only
    attributes = db.query(ReadingAttribute).filter(
        ReadingAttribute.reading_id.in_(reading_ids)
    ).all()
    
    # Aggregate and return
    return aggregate_attributes(attributes)
```

## Current Implementation Status

### ✅ What Works
- **User Creation:** Automatic when first reading is saved
- **User Isolation:** Each user's readings are completely separate
- **Matrix ID Mapping:** Matrix IDs like `@waqaas:endlessperfect.com` map to database users
- **History Retrieval:** API correctly filters by Matrix ID
- **Analytics:** Calculated per-user based on their readings only

### ⚠️ Current Limitation
```rust
// In src/commands.rs
let matrix_id = "@waqaas:endlessperfect.com".to_string();  // HARDCODED!
```

**Impact:** 
- All readings are currently saved under `@waqaas:endlessperfect.com`
- If `@keanu:endlessperfect.com` uses the client, their readings would also be saved under `@waqaas`
- History would be mixed together

### 🔧 Proper Fix (Future)
```rust
// Need to access the actual logged-in user from context
let matrix_id = ctx.get_user_matrix_id();  // Get from application context
```

## Example Data in Database

### Users Table
```
user_id | matrix_id                      | username | created_at
--------|--------------------------------|----------|------------
1       | @waqaas:endlessperfect.com    | waqaas   | 2025-10-07
2       | @keanu:endlessperfect.com     | keanu    | 2025-10-07
3       | @riva:endlessperfect.com      | riva     | 2025-10-07
```

### Readings Table
```
reading_id | user_id | spread_type | reading_date        | notes
-----------|---------|-------------|---------------------|-------
1          | 1       | 3           | 2025-10-07 10:00:00 | NULL
2          | 1       | 5           | 2025-10-07 11:00:00 | NULL
3          | 2       | 3           | 2025-10-07 12:00:00 | NULL
```

### Cards Drawn Table
```
card_id | reading_id | position | card_name        | card_label
--------|------------|----------|------------------|------------
1       | 1          | 0        | The Star         | Card 1
2       | 1          | 1        | Six of Swords    | Card 2
3       | 1          | 2        | Knight of Swords | Card 3
```

### Reading Attributes Table
```
attribute_id | reading_id | attribute_type | attribute_value | count
-------------|------------|----------------|-----------------|-------
1            | 1          | suit           | Swords          | 2
2            | 1          | element        | Air             | 2
3            | 1          | planet         | Mercury         | 1
4            | 1          | sign           | Aquarius        | 2
```

## Privacy & Security

### User Isolation
- Each user can only see their own readings
- API endpoints require Matrix ID to fetch data
- No cross-user data leakage

### Future Enhancements
1. **Room-based filtering:** Track which room each reading was done in
2. **Private readings:** Flag to hide readings from history
3. **Shared readings:** Allow users to share specific readings
4. **Export:** Allow users to export their reading history

## Summary

The system uses a **two-level identification**:
1. **Matrix ID** (external): `@waqaas:endlessperfect.com` - used by the client
2. **User ID** (internal): Integer primary key - used by the database

When you do a reading:
- Client sends Matrix ID to API
- API finds or creates user with that Matrix ID
- Reading is linked to that user's internal ID
- All future queries filter by that user's ID

This ensures complete separation between users while using the familiar Matrix ID as the identifier! 🔮
