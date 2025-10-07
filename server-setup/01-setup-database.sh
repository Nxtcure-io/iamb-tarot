#!/bin/bash

# Setup script for tarot readings database
# Run this on the AWS Lightsail server

set -e

echo "=========================================="
echo "Tarot Readings Database Setup"
echo "=========================================="
echo ""

# Database configuration
DB_NAME="tarot_readings"
DB_USER="tarot_user"
DB_PASSWORD="tarot_secure_password_$(openssl rand -hex 8)"

echo "Creating database and user..."
echo "Database: $DB_NAME"
echo "User: $DB_USER"
echo "Password: $DB_PASSWORD"
echo ""

# Create database and user in PostgreSQL container
docker exec matrix-postgres-1 psql -U synapse -c "CREATE DATABASE $DB_NAME;"
docker exec matrix-postgres-1 psql -U synapse -c "CREATE USER $DB_USER WITH PASSWORD '$DB_PASSWORD';"
docker exec matrix-postgres-1 psql -U synapse -c "GRANT ALL PRIVILEGES ON DATABASE $DB_NAME TO $DB_USER;"
docker exec matrix-postgres-1 psql -U synapse -d $DB_NAME -c "GRANT ALL ON SCHEMA public TO $DB_USER;"

echo "✓ Database and user created"
echo ""

# Save credentials
CREDS_FILE="$HOME/matrix/tarot-api/.env"
mkdir -p "$HOME/matrix/tarot-api"

cat > "$CREDS_FILE" << EOF
# Tarot API Configuration
DATABASE_URL=postgresql://$DB_USER:$DB_PASSWORD@postgres:5432/$DB_NAME
MATRIX_SERVER=https://endlessperfect.com
API_SECRET_KEY=$(openssl rand -hex 32)
CORS_ORIGINS=*

# Database credentials (for reference)
DB_NAME=$DB_NAME
DB_USER=$DB_USER
DB_PASSWORD=$DB_PASSWORD
EOF

chmod 600 "$CREDS_FILE"

echo "✓ Credentials saved to: $CREDS_FILE"
echo ""

# Initialize schema
echo "Initializing database schema..."

docker exec matrix-postgres-1 psql -U $DB_USER -d $DB_NAME << 'EOSQL'

-- Users table
CREATE TABLE IF NOT EXISTS users (
    user_id SERIAL PRIMARY KEY,
    matrix_id VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_reading_at TIMESTAMP
);

-- Readings table
CREATE TABLE IF NOT EXISTS readings (
    reading_id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(user_id) ON DELETE CASCADE,
    room_id VARCHAR(255),
    spread_type VARCHAR(50) NOT NULL,
    reading_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    notes TEXT,
    is_private BOOLEAN DEFAULT false
);

-- Cards drawn in each reading
CREATE TABLE IF NOT EXISTS cards_drawn (
    card_id SERIAL PRIMARY KEY,
    reading_id INTEGER REFERENCES readings(reading_id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    card_name VARCHAR(100) NOT NULL,
    card_label VARCHAR(100),
    is_reversed BOOLEAN DEFAULT false,
    interpretation TEXT
);

-- Card database (reference data)
CREATE TABLE IF NOT EXISTS card_database (
    card_db_id SERIAL PRIMARY KEY,
    card_name VARCHAR(100) UNIQUE NOT NULL,
    card_number VARCHAR(10),
    card_title VARCHAR(100) NOT NULL,
    arcana_type VARCHAR(20) NOT NULL,
    suit VARCHAR(20),
    upright_meaning TEXT,
    reversed_meaning TEXT,
    keywords TEXT[],
    image_path VARCHAR(255)
);

-- Spread templates
CREATE TABLE IF NOT EXISTS spread_templates (
    template_id SERIAL PRIMARY KEY,
    template_name VARCHAR(100) UNIQUE NOT NULL,
    card_count INTEGER NOT NULL,
    positions JSONB NOT NULL,
    description TEXT,
    is_public BOOLEAN DEFAULT true,
    created_by INTEGER REFERENCES users(user_id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_readings_user_id ON readings(user_id);
CREATE INDEX IF NOT EXISTS idx_readings_date ON readings(reading_date DESC);
CREATE INDEX IF NOT EXISTS idx_cards_drawn_reading_id ON cards_drawn(reading_id);
CREATE INDEX IF NOT EXISTS idx_users_matrix_id ON users(matrix_id);

EOSQL

echo "✓ Schema initialized"
echo ""

# Seed card database with Major Arcana
echo "Seeding card database..."

docker exec matrix-postgres-1 psql -U $DB_USER -d $DB_NAME << 'EOSQL'

INSERT INTO card_database (card_name, card_number, card_title, arcana_type, upright_meaning, reversed_meaning, keywords) VALUES
('fool', '0', 'THE FOOL', 'major', 
 'New beginnings, innocence, spontaneity, free spirit, adventure',
 'Recklessness, taken advantage of, inconsideration, naivety',
 ARRAY['beginnings', 'innocence', 'spontaneity', 'free spirit']),

('magician', 'I', 'THE MAGICIAN', 'major',
 'Manifestation, resourcefulness, power, inspired action',
 'Manipulation, poor planning, untapped talents',
 ARRAY['manifestation', 'power', 'action', 'resourcefulness']),

('high-priestess', 'II', 'THE HIGH PRIESTESS', 'major',
 'Intuition, sacred knowledge, divine feminine, subconscious mind',
 'Secrets, disconnected from intuition, withdrawal',
 ARRAY['intuition', 'mystery', 'subconscious', 'wisdom']),

('empress', 'III', 'THE EMPRESS', 'major',
 'Femininity, beauty, nature, nurturing, abundance',
 'Creative block, dependence on others, emptiness',
 ARRAY['abundance', 'nurturing', 'fertility', 'nature']),

('emperor', 'IV', 'THE EMPEROR', 'major',
 'Authority, establishment, structure, father figure',
 'Domination, excessive control, lack of discipline',
 ARRAY['authority', 'structure', 'control', 'father']),

('hierophant', 'V', 'THE HIEROPHANT', 'major',
 'Spiritual wisdom, religious beliefs, conformity, tradition',
 'Personal beliefs, freedom, challenging the status quo',
 ARRAY['tradition', 'conformity', 'spirituality', 'education']),

('lovers', 'VI', 'THE LOVERS', 'major',
 'Love, harmony, relationships, values alignment, choices',
 'Self-love, disharmony, imbalance, misalignment of values',
 ARRAY['love', 'harmony', 'relationships', 'choices']),

('chariot', 'VII', 'THE CHARIOT', 'major',
 'Control, willpower, success, action, determination',
 'Self-discipline, opposition, lack of direction',
 ARRAY['willpower', 'determination', 'control', 'victory']),

('strength', 'VIII', 'STRENGTH', 'major',
 'Strength, courage, persuasion, influence, compassion',
 'Inner strength, self-doubt, low energy, raw emotion',
 ARRAY['courage', 'compassion', 'inner strength', 'patience']),

('hermit', 'IX', 'THE HERMIT', 'major',
 'Soul searching, introspection, being alone, inner guidance',
 'Isolation, loneliness, withdrawal',
 ARRAY['introspection', 'solitude', 'wisdom', 'guidance']),

('wheel-of-fortune', 'X', 'WHEEL OF FORTUNE', 'major',
 'Good luck, karma, life cycles, destiny, turning point',
 'Bad luck, resistance to change, breaking cycles',
 ARRAY['destiny', 'cycles', 'change', 'fortune']),

('justice', 'XI', 'JUSTICE', 'major',
 'Justice, fairness, truth, cause and effect, law',
 'Unfairness, lack of accountability, dishonesty',
 ARRAY['justice', 'fairness', 'truth', 'law']),

('hanged-man', 'XII', 'THE HANGED MAN', 'major',
 'Pause, surrender, letting go, new perspectives',
 'Delays, resistance, stalling, indecision',
 ARRAY['surrender', 'perspective', 'pause', 'sacrifice']),

('death', 'XIII', 'DEATH', 'major',
 'Endings, change, transformation, transition',
 'Resistance to change, personal transformation, inner purging',
 ARRAY['transformation', 'endings', 'change', 'transition']),

('temperance', 'XIV', 'TEMPERANCE', 'major',
 'Balance, moderation, patience, purpose',
 'Imbalance, excess, self-healing, re-alignment',
 ARRAY['balance', 'moderation', 'patience', 'harmony']),

('devil', 'XV', 'THE DEVIL', 'major',
 'Shadow self, attachment, addiction, restriction, sexuality',
 'Releasing limiting beliefs, exploring dark thoughts, detachment',
 ARRAY['bondage', 'addiction', 'materialism', 'shadow']),

('tower', 'XVI', 'THE TOWER', 'major',
 'Sudden change, upheaval, chaos, revelation, awakening',
 'Personal transformation, fear of change, averting disaster',
 ARRAY['upheaval', 'revelation', 'chaos', 'awakening']),

('star', 'XVII', 'THE STAR', 'major',
 'Hope, faith, purpose, renewal, spirituality',
 'Lack of faith, despair, self-trust, disconnection',
 ARRAY['hope', 'faith', 'renewal', 'inspiration']),

('moon', 'XVIII', 'THE MOON', 'major',
 'Illusion, fear, anxiety, subconscious, intuition',
 'Release of fear, repressed emotion, inner confusion',
 ARRAY['illusion', 'intuition', 'dreams', 'subconscious']),

('sun', 'XIX', 'THE SUN', 'major',
 'Positivity, fun, warmth, success, vitality',
 'Inner child, feeling down, overly optimistic',
 ARRAY['joy', 'success', 'vitality', 'positivity']),

('judgement', 'XX', 'JUDGEMENT', 'major',
 'Judgement, rebirth, inner calling, absolution',
 'Self-doubt, inner critic, ignoring the call',
 ARRAY['judgement', 'rebirth', 'calling', 'absolution']),

('world', 'XXI', 'THE WORLD', 'major',
 'Completion, integration, accomplishment, travel',
 'Seeking personal closure, short-cuts, delays',
 ARRAY['completion', 'accomplishment', 'travel', 'fulfillment'])

ON CONFLICT (card_name) DO NOTHING;

EOSQL

echo "✓ Card database seeded with 22 Major Arcana cards"
echo ""

# Seed default spread templates
echo "Creating default spread templates..."

docker exec matrix-postgres-1 psql -U $DB_USER -d $DB_NAME << 'EOSQL'

INSERT INTO spread_templates (template_name, card_count, positions, description, is_public) VALUES
('single', 1, 
 '[{"position": 0, "label": "Card of the Day"}]'::jsonb,
 'Single card reading for daily guidance',
 true),

('threecard', 3,
 '[{"position": 0, "label": "Past"}, {"position": 1, "label": "Present"}, {"position": 2, "label": "Future"}]'::jsonb,
 'Classic three-card spread for past, present, and future',
 true),

('celtic-cross', 10,
 '[
   {"position": 0, "label": "Present Situation"},
   {"position": 1, "label": "Challenge"},
   {"position": 2, "label": "Past"},
   {"position": 3, "label": "Future"},
   {"position": 4, "label": "Above"},
   {"position": 5, "label": "Below"},
   {"position": 6, "label": "Advice"},
   {"position": 7, "label": "External Influences"},
   {"position": 8, "label": "Hopes and Fears"},
   {"position": 9, "label": "Outcome"}
 ]'::jsonb,
 'Traditional Celtic Cross spread for comprehensive readings',
 true)

ON CONFLICT (template_name) DO NOTHING;

EOSQL

echo "✓ Default spread templates created"
echo ""

echo "=========================================="
echo "Setup Complete!"
echo "=========================================="
echo ""
echo "Database: $DB_NAME"
echo "User: $DB_USER"
echo "Credentials saved to: $CREDS_FILE"
echo ""
echo "Next steps:"
echo "1. Review credentials in $CREDS_FILE"
echo "2. Set up Tarot API service (run 02-setup-api.sh)"
echo "3. Update nginx configuration"
echo ""
