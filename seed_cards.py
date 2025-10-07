#!/usr/bin/env python3
"""
Script to seed the tarot card database from cards.csv
"""

import csv
import psycopg2
from psycopg2.extras import RealDictCursor

# Database connection parameters
DB_CONFIG = {
    'host': 'postgres',  # Using docker network name
    'port': 5432,
    'database': 'tarot_readings',
    'user': 'tarot_user',
    'password': 'tarot_secure_password_bea6b8ff05a98a3e'
}

def create_card_database_table(conn):
    """Create the card_database table if it doesn't exist"""
    create_table_sql = """
    CREATE TABLE IF NOT EXISTS card_database (
        card_db_id SERIAL PRIMARY KEY,
        card_name VARCHAR(100) UNIQUE NOT NULL,
        title VARCHAR(100),
        image VARCHAR(255),
        planet_orb VARCHAR(50),
        planet_house VARCHAR(50),
        sign_1 VARCHAR(50),
        sign_2 VARCHAR(50),
        sign_3 VARCHAR(50),
        element_1 VARCHAR(50),
        element_2 VARCHAR(50),
        suit_1 VARCHAR(50),
        suit_2 VARCHAR(50),
        path VARCHAR(50),
        sephira VARCHAR(50),
        info TEXT,
        deepinfo TEXT
    );
    """
    
    with conn.cursor() as cur:
        cur.execute(create_table_sql)
        conn.commit()
        print("✓ Created card_database table")

def load_cards_from_csv(filename='cards.csv'):
    """Load cards from CSV file"""
    cards = []
    with open(filename, 'r', encoding='utf-8') as f:
        reader = csv.DictReader(f)
        for row in reader:
            # Clean up empty strings
            card = {k: v if v else None for k, v in row.items()}
            cards.append(card)
    return cards

def insert_cards(conn, cards):
    """Insert cards into the database"""
    insert_sql = """
    INSERT INTO card_database (
        card_name, title, image, planet_orb, planet_house,
        sign_1, sign_2, sign_3, element_1, element_2,
        suit_1, suit_2, path, sephira, info, deepinfo
    ) VALUES (
        %(Card)s, %(Title)s, %(Image)s, %(planet_orb)s, %(planet_house)s,
        %(sign_1)s, %(sign_2)s, %(sign_3)s, %(element_1)s, %(element_2)s,
        %(suit_1)s, %(suit_2)s, %(path)s, %(sephira)s, %(info)s, %(deepinfo)s
    )
    ON CONFLICT (card_name) DO UPDATE SET
        title = EXCLUDED.title,
        image = EXCLUDED.image,
        planet_orb = EXCLUDED.planet_orb,
        planet_house = EXCLUDED.planet_house,
        sign_1 = EXCLUDED.sign_1,
        sign_2 = EXCLUDED.sign_2,
        sign_3 = EXCLUDED.sign_3,
        element_1 = EXCLUDED.element_1,
        element_2 = EXCLUDED.element_2,
        suit_1 = EXCLUDED.suit_1,
        suit_2 = EXCLUDED.suit_2,
        path = EXCLUDED.path,
        sephira = EXCLUDED.sephira,
        info = EXCLUDED.info,
        deepinfo = EXCLUDED.deepinfo;
    """
    
    with conn.cursor() as cur:
        for card in cards:
            try:
                cur.execute(insert_sql, card)
            except Exception as e:
                print(f"Error inserting {card.get('Card', 'unknown')}: {e}")
                conn.rollback()
                continue
        conn.commit()
    
    print(f"✓ Inserted/updated {len(cards)} cards")

def verify_data(conn):
    """Verify the data was loaded correctly"""
    with conn.cursor(cursor_factory=RealDictCursor) as cur:
        # Count cards
        cur.execute("SELECT COUNT(*) as count FROM card_database")
        count = cur.fetchone()['count']
        print(f"\n✓ Total cards in database: {count}")
        
        # Show sample of cards
        cur.execute("""
            SELECT card_name, title, sign_1, element_1, suit_1 
            FROM card_database 
            ORDER BY card_db_id 
            LIMIT 5
        """)
        print("\n✓ Sample cards:")
        for card in cur.fetchall():
            print(f"  - {card['card_name']}: {card.get('title', 'No title')}")
        
        # Count cards by suit
        cur.execute("""
            SELECT suit_1, COUNT(*) as count 
            FROM card_database 
            WHERE suit_1 IS NOT NULL 
            GROUP BY suit_1
        """)
        print("\n✓ Cards by suit:")
        for row in cur.fetchall():
            print(f"  - {row['suit_1']}: {row['count']} cards")
        
        # Count cards by element
        cur.execute("""
            SELECT element_1, COUNT(*) as count 
            FROM card_database 
            WHERE element_1 IS NOT NULL 
            GROUP BY element_1
        """)
        print("\n✓ Cards by element:")
        for row in cur.fetchall():
            print(f"  - {row['element_1']}: {row['count']} cards")

def main():
    """Main function to seed the database"""
    print("🔮 Tarot Card Database Seeder\n")
    
    try:
        # Connect to database
        print(f"Connecting to database at {DB_CONFIG['host']}...")
        conn = psycopg2.connect(**DB_CONFIG)
        print("✓ Connected to database\n")
        
        # Create table
        create_card_database_table(conn)
        
        # Load cards from CSV
        print("\nLoading cards from cards.csv...")
        cards = load_cards_from_csv()
        print(f"✓ Loaded {len(cards)} cards from CSV\n")
        
        # Insert cards
        print("Inserting cards into database...")
        insert_cards(conn, cards)
        
        # Verify
        verify_data(conn)
        
        print("\n✅ Database seeding complete!")
        
    except Exception as e:
        print(f"\n❌ Error: {e}")
        return 1
    finally:
        if 'conn' in locals():
            conn.close()
    
    return 0

if __name__ == "__main__":
    exit(main())
