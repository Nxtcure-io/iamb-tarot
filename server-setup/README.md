# Tarot Readings Server Setup

Complete setup guide for deploying the Tarot Readings tracking system on your Matrix server.

## Prerequisites

- AWS Lightsail instance running (3.15.195.126)
- Matrix Synapse server deployed
- PostgreSQL container running
- SSH access to server
- Domain: endlessperfect.com

## Architecture Overview

```
┌─────────────────────────────────────────────┐
│  AWS Lightsail (3.15.195.126)              │
│                                             │
│  ┌─────────────────────────────────────┐   │
│  │  Docker Containers                  │   │
│  │                                     │   │
│  │  ┌──────────┐  ┌──────────────┐   │   │
│  │  │ Synapse  │  │  PostgreSQL  │   │   │
│  │  │  :8008   │  │    :5432     │   │   │
│  │  │          │  │              │   │   │
│  │  │          │  │ - synapse DB │   │   │
│  │  │          │  │ - tarot DB   │   │   │
│  │  └──────────┘  └──────────────┘   │   │
│  │                                     │   │
│  │  ┌──────────┐  ┌──────────────┐   │   │
│  │  │ Tarot API│  │    Nginx     │   │   │
│  │  │  :8080   │  │  :80, :443   │   │   │
│  │  └──────────┘  └──────────────┘   │   │
│  └─────────────────────────────────────┘   │
└─────────────────────────────────────────────┘
```

## Setup Steps

### 1. Transfer Setup Scripts to Server

From your local machine:

```bash
# Copy setup scripts to server
scp -i /home/waqaas/antler.pem -r server-setup/ ubuntu@3.15.195.126:~/

# SSH into server
ssh -i /home/waqaas/antler.pem ubuntu@3.15.195.126
```

### 2. Run Database Setup

```bash
cd ~/server-setup
chmod +x *.sh

# Create tarot_readings database and user
./01-setup-database.sh
```

This script will:
- Create `tarot_readings` database
- Create `tarot_user` with secure password
- Initialize database schema (users, readings, cards, etc.)
- Seed card database with 22 Major Arcana cards
- Create default spread templates
- Save credentials to `~/matrix/tarot-api/.env`

### 3. Setup API Service

```bash
# Create API application files
./02-setup-api.sh
```

This script creates:
- FastAPI application structure
- Database models and schemas
- API routes for readings, users, cards
- Matrix registration endpoint
- Dockerfile for containerization

### 4. Update Docker Compose

```bash
# Add Tarot API to docker-compose.yml
./03-update-docker-compose.sh
```

This script:
- Backs up existing docker-compose.yml
- Adds tarot-api service configuration
- Creates nginx reverse proxy config

### 5. Manual Configuration

#### Update Nginx Configuration

Edit the main nginx config:

```bash
nano ~/matrix/nginx/matrix.conf
```

Add inside the `server` block:

```nginx
# Include Tarot API routes
include /etc/nginx/conf.d/tarot-api.conf;
```

#### Build and Start Services

```bash
cd ~/matrix

# Build Tarot API image
docker compose build tarot-api

# Start Tarot API service
docker compose up -d tarot-api

# Restart nginx to load new config
docker compose restart nginx
```

### 6. Verify Installation

```bash
# Check service status
docker compose ps

# Check Tarot API logs
docker compose logs -f tarot-api

# Test health endpoint
curl https://endlessperfect.com/tarot-api/health

# Test API root
curl https://endlessperfect.com/tarot-api/
```

Expected response:
```json
{
  "service": "Tarot Readings API",
  "version": "1.0.0",
  "status": "running"
}
```

## API Endpoints

### Base URL
```
https://endlessperfect.com/tarot-api/api
```

### User Management

**Create User**
```bash
POST /api/users
{
  "matrix_id": "@user:endlessperfect.com",
  "username": "user"
}
```

**Get User**
```bash
GET /api/users/@user:endlessperfect.com
```

### Reading Management

**Save Reading**
```bash
POST /api/readings
{
  "matrix_id": "@user:endlessperfect.com",
  "room_id": "!abc123:endlessperfect.com",
  "spread_type": "threecard",
  "cards": [
    {
      "position": 0,
      "card_name": "fool",
      "card_label": "Past",
      "is_reversed": false
    },
    {
      "position": 1,
      "card_name": "tower",
      "card_label": "Present",
      "is_reversed": false
    },
    {
      "position": 2,
      "card_name": "star",
      "card_label": "Future",
      "is_reversed": false
    }
  ],
  "notes": "Interesting reading about change",
  "is_private": false
}
```

**Get User Readings**
```bash
GET /api/readings/user/@user:endlessperfect.com?limit=50
```

**Get Specific Reading**
```bash
GET /api/readings/123
```

**Update Reading Notes**
```bash
PUT /api/readings/123
{
  "notes": "Updated interpretation"
}
```

**Delete Reading**
```bash
DELETE /api/readings/123
```

### Card Database

**Get All Cards**
```bash
GET /api/cards
```

**Get Card Info**
```bash
GET /api/cards/fool
```

Response:
```json
{
  "card_name": "fool",
  "card_title": "THE FOOL",
  "card_number": "0",
  "arcana_type": "major",
  "upright_meaning": "New beginnings, innocence, spontaneity...",
  "reversed_meaning": "Recklessness, taken advantage of...",
  "keywords": ["beginnings", "innocence", "spontaneity"]
}
```

### Spread Templates

**Get All Spreads**
```bash
GET /api/spreads
```

### Matrix Integration

**Register New Matrix User**
```bash
POST /api/matrix/register
{
  "username": "newuser",
  "password": "SecurePassword123!",
  "admin": false
}
```

Response:
```json
{
  "matrix_id": "@newuser:endlessperfect.com",
  "username": "newuser",
  "success": true,
  "message": "User registered successfully"
}
```

## Database Schema

### Tables Created

1. **users** - Matrix users who use tarot readings
2. **readings** - Individual tarot reading sessions
3. **cards_drawn** - Cards drawn in each reading
4. **card_database** - Reference data for all tarot cards
5. **spread_templates** - Predefined spread configurations

### Seeded Data

**Card Database:**
- 22 Major Arcana cards
- Card meanings (upright and reversed)
- Keywords for each card

**Spread Templates:**
- Single card reading
- Three-card spread (Past/Present/Future)
- Celtic Cross (10 cards)

## Configuration Files

### Environment Variables

Located at: `~/matrix/tarot-api/.env`

```bash
DATABASE_URL=postgresql://tarot_user:PASSWORD@postgres:5432/tarot_readings
MATRIX_SERVER=https://endlessperfect.com
API_SECRET_KEY=RANDOM_SECRET_KEY
CORS_ORIGINS=*
```

### Docker Compose

Service configuration in `~/matrix/docker-compose.yml`:

```yaml
tarot-api:
  build: ./tarot-api
  container_name: tarot-api
  ports:
    - "8080:8080"
  environment:
    - DATABASE_URL=postgresql://tarot_user:PASSWORD@postgres:5432/tarot_readings
    - MATRIX_SERVER=https://endlessperfect.com
  depends_on:
    - postgres
  restart: unless-stopped
```

## Troubleshooting

### Service Won't Start

Check logs:
```bash
docker compose logs tarot-api
```

Common issues:
- Database connection failed: Check credentials in `.env`
- Port already in use: Change port in docker-compose.yml
- Build failed: Check Python dependencies

### Database Connection Issues

Test database connection:
```bash
docker exec matrix-postgres-1 psql -U tarot_user -d tarot_readings -c "SELECT COUNT(*) FROM users;"
```

### API Returns 502 Bad Gateway

- Check if tarot-api container is running: `docker compose ps`
- Check nginx logs: `docker compose logs nginx`
- Verify nginx config includes tarot-api.conf

### Can't Register Matrix Users

- Check Synapse registration is enabled in `homeserver.yaml`
- Verify Matrix server is accessible
- Check API logs for detailed error

## Security Considerations

1. **Database Password**: Auto-generated secure password
2. **API Secret Key**: Random 32-byte key for JWT tokens
3. **CORS**: Currently set to `*` for development, restrict in production
4. **HTTPS**: All traffic through nginx with SSL
5. **Rate Limiting**: Consider adding to nginx config

## Maintenance

### Backup Database

```bash
# Backup tarot_readings database
docker exec matrix-postgres-1 pg_dump -U tarot_user tarot_readings > tarot_backup_$(date +%Y%m%d).sql

# Restore from backup
docker exec -i matrix-postgres-1 psql -U tarot_user -d tarot_readings < tarot_backup_20251007.sql
```

### View Logs

```bash
# Tarot API logs
docker compose logs -f tarot-api

# All services
docker compose logs -f
```

### Restart Services

```bash
# Restart Tarot API only
docker compose restart tarot-api

# Restart all services
docker compose restart
```

### Update API Code

```bash
cd ~/matrix/tarot-api

# Edit files as needed
nano app.py

# Rebuild and restart
cd ~/matrix
docker compose build tarot-api
docker compose up -d tarot-api
```

## Next Steps

1. **Test API endpoints** using curl or Postman
2. **Integrate with iamb client** (see IAMB_INTEGRATION.md)
3. **Add more card data** (Minor Arcana)
4. **Create custom spreads**
5. **Add user authentication** (JWT tokens)
6. **Implement reading sharing**
7. **Add analytics/statistics**

## Support

For issues:
1. Check logs: `docker compose logs tarot-api`
2. Verify database connection
3. Test API endpoints with curl
4. Review nginx configuration

## Resources

- FastAPI Documentation: https://fastapi.tiangolo.com/
- SQLAlchemy Documentation: https://docs.sqlalchemy.org/
- Matrix API Spec: https://spec.matrix.org/
- PostgreSQL Documentation: https://www.postgresql.org/docs/
