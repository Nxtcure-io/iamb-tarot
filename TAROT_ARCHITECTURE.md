# Tarot Readings Tracker - Architecture

## Overview

A persistent tarot reading tracking system integrated with Matrix server for user authentication and reading history.

## System Components

### 1. Matrix Server (Existing)
- **Server**: endlessperfect.com
- **Location**: AWS Lightsail (3.15.195.126)
- **Directory**: `~/matrix/`
- **Purpose**: User authentication, messaging, card delivery

### 2. PostgreSQL Database (Existing + New DB)
- **Container**: matrix-postgres-1
- **Existing DB**: `synapse` (Matrix data)
- **New DB**: `tarot_readings` (Reading history)
- **User**: `tarot_user`
- **Port**: 5432 (internal)

### 3. Tarot API Service (New)
- **Technology**: FastAPI (Python)
- **Port**: 8080
- **Purpose**: 
  - User registration
  - Reading history management
  - Card interpretation database
  - Spread templates

### 4. iamb Client (Modified)
- **Purpose**: 
  - Post cards to Matrix rooms
  - Trigger reading saves via API
  - Display reading history

## Database Schema

### Database: `tarot_readings`

#### Table: `users`
```sql
CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    matrix_id VARCHAR(255) UNIQUE NOT NULL,  -- @user:endlessperfect.com
    username VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_reading_at TIMESTAMP
);
```

#### Table: `readings`
```sql
CREATE TABLE readings (
    reading_id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(user_id),
    room_id VARCHAR(255),  -- Matrix room ID
    spread_type VARCHAR(50),  -- 'single', 'threecard', 'celtic-cross', etc.
    reading_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    notes TEXT,
    is_private BOOLEAN DEFAULT false
);
```

#### Table: `cards_drawn`
```sql
CREATE TABLE cards_drawn (
    card_id SERIAL PRIMARY KEY,
    reading_id INTEGER REFERENCES readings(reading_id),
    position INTEGER,  -- Position in spread (0, 1, 2 for three-card)
    card_name VARCHAR(100),  -- 'fool', 'magician', etc.
    card_label VARCHAR(100),  -- 'Past', 'Present', 'Future'
    is_reversed BOOLEAN DEFAULT false,
    interpretation TEXT
);
```

#### Table: `card_database`
```sql
CREATE TABLE card_database (
    card_db_id SERIAL PRIMARY KEY,
    card_name VARCHAR(100) UNIQUE NOT NULL,
    card_number VARCHAR(10),  -- '0', 'I', 'II', etc.
    card_title VARCHAR(100),  -- 'THE FOOL', 'THE MAGICIAN'
    arcana_type VARCHAR(20),  -- 'major', 'minor'
    suit VARCHAR(20),  -- NULL for major, 'wands', 'cups', 'swords', 'pentacles'
    upright_meaning TEXT,
    reversed_meaning TEXT,
    keywords TEXT[],
    image_path VARCHAR(255)
);
```

#### Table: `spread_templates`
```sql
CREATE TABLE spread_templates (
    template_id SERIAL PRIMARY KEY,
    template_name VARCHAR(100) UNIQUE NOT NULL,
    card_count INTEGER NOT NULL,
    positions JSONB,  -- [{"position": 0, "label": "Past"}, ...]
    description TEXT,
    is_public BOOLEAN DEFAULT true
);
```

## API Endpoints

### User Management
- `POST /api/register` - Register new Matrix user
- `POST /api/login` - Verify Matrix credentials
- `GET /api/users/{matrix_id}` - Get user info

### Reading Management
- `POST /api/readings` - Save a new reading
- `GET /api/readings/{user_id}` - Get user's reading history
- `GET /api/readings/{reading_id}` - Get specific reading
- `PUT /api/readings/{reading_id}` - Update reading notes
- `DELETE /api/readings/{reading_id}` - Delete reading

### Card Database
- `GET /api/cards` - List all cards
- `GET /api/cards/{card_name}` - Get card details
- `GET /api/cards/{card_name}/meaning` - Get interpretation

### Spread Templates
- `GET /api/spreads` - List available spreads
- `GET /api/spreads/{template_name}` - Get spread details
- `POST /api/spreads` - Create custom spread

### Matrix Integration
- `POST /api/matrix/register` - Create Matrix account
- `POST /api/matrix/verify` - Verify Matrix credentials

## Data Flow

### New Reading Flow
```
1. User types: :tarot threecard
2. iamb selects 3 random cards
3. iamb posts cards to Matrix room
4. iamb calls API: POST /api/readings
   {
     "matrix_id": "@user:endlessperfect.com",
     "room_id": "!abc123:endlessperfect.com",
     "spread_type": "threecard",
     "cards": [
       {"position": 0, "card_name": "fool", "label": "Past"},
       {"position": 1, "card_name": "tower", "label": "Present"},
       {"position": 2, "card_name": "star", "label": "Future"}
     ]
   }
5. API saves to database
6. Returns reading_id
```

### View History Flow
```
1. User types: :tarot history
2. iamb calls API: GET /api/readings/{user_id}
3. API returns list of readings
4. iamb displays in scrollback:
   - Date
   - Spread type
   - Cards drawn
   - Notes
```

## Deployment Structure

```
AWS Lightsail (3.15.195.126)
├── ~/matrix/
│   ├── docker-compose.yml (updated)
│   ├── synapse/ (Matrix server)
│   ├── postgres/ (PostgreSQL data)
│   ├── nginx/ (Reverse proxy)
│   └── tarot-api/ (NEW)
│       ├── app.py (FastAPI app)
│       ├── models.py (Database models)
│       ├── routes.py (API endpoints)
│       ├── requirements.txt
│       └── Dockerfile
```

## Docker Compose Updates

Add to `~/matrix/docker-compose.yml`:

```yaml
  tarot-api:
    build: ./tarot-api
    container_name: tarot-api
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgresql://tarot_user:tarot_password@postgres:5432/tarot_readings
      - MATRIX_SERVER=https://endlessperfect.com
    depends_on:
      - postgres
    restart: unless-stopped
```

## Security Considerations

1. **Authentication**: Use Matrix access tokens for API auth
2. **Database**: Separate user (`tarot_user`) with limited permissions
3. **API**: Rate limiting on registration endpoint
4. **Privacy**: Users can mark readings as private
5. **HTTPS**: All API calls through nginx reverse proxy

## Implementation Phases

### Phase 1: Database Setup (Current)
- [ ] Create `tarot_readings` database
- [ ] Create `tarot_user` PostgreSQL user
- [ ] Initialize schema
- [ ] Seed card database with Major Arcana

### Phase 2: API Development
- [ ] FastAPI application structure
- [ ] Database models (SQLAlchemy)
- [ ] User registration endpoint
- [ ] Reading CRUD endpoints
- [ ] Matrix integration

### Phase 3: iamb Integration
- [ ] Add API client to iamb
- [ ] `:tarot history` command
- [ ] `:tarot save` command
- [ ] Auto-save readings option
- [ ] Display card meanings

### Phase 4: Enhanced Features
- [ ] Custom spread creator
- [ ] Reading journal/notes
- [ ] Card interpretation AI
- [ ] Reading sharing
- [ ] Statistics/analytics

## Configuration

### Environment Variables

**Tarot API** (`.env`):
```bash
DATABASE_URL=postgresql://tarot_user:tarot_password@postgres:5432/tarot_readings
MATRIX_SERVER=https://endlessperfect.com
MATRIX_ADMIN_TOKEN=<admin_token>
API_SECRET_KEY=<random_secret>
CORS_ORIGINS=*
```

**iamb** (`config.toml`):
```toml
[settings.tarot]
api_url = "https://endlessperfect.com/tarot-api"
auto_save_readings = true
show_interpretations = true
```

## Next Steps

1. Set up tarot_readings database
2. Create tarot_user with permissions
3. Initialize schema
4. Build FastAPI application
5. Deploy as Docker container
6. Update nginx config for /tarot-api route
7. Integrate with iamb client

## Benefits

- **Persistent History**: Never lose a reading
- **Pattern Recognition**: See trends over time
- **Learning Tool**: Build interpretation skills
- **Sharing**: Share readings with others
- **Privacy**: Keep personal readings private
- **Analytics**: Track card frequencies, patterns
