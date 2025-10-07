# Tarot iamb - Quick Reference

## 🎴 Tarot Reading Commands

### Single Card Lookups (Not Saved)
```
:tarot fool                 # Look up "The Fool" card
:tarot six swords          # Look up "Six of Swords"
:tarot science             # Look up by title
:tarot fool info           # With card meanings
:tarot fool deepinfo       # With deep meanings
```

### Multi-Card Spreads (Auto-Saved)
```
:tarot 3                   # 3-card spread
:tarot 5                   # 5-card spread
:tarot 7                   # 7-card spread
:tarot 3 info              # 3-card spread with meanings
```

## 📊 History & Analytics Commands

### View History
```
:tarothistory              # List all your readings
:tarothistory 1            # View reading #1 details
:tarothistory 1 info       # View reading #1 with card meanings
```

### Analytics & Graphs
```
:tarothistory suits        # Suit distribution bar graph
:tarothistory elements     # Element distribution
:tarothistory planets      # Planet distribution
:tarothistory signs        # Zodiac sign distribution
:tarothistory sephira      # Sephira distribution
:tarothistory summary      # Overall statistics
```

## 🔧 Server Management

### SSH Access
```bash
ssh -i /home/waqaas/antler.pem ubuntu@3.15.195.126
```

### Check Services
```bash
cd ~/matrix
docker compose ps
docker logs tarot-api --tail 50
```

### Database Access
```bash
docker exec matrix-postgres-1 psql -U tarot_user -d tarot_readings -c "SELECT COUNT(*) FROM card_database"
```

### API Testing
```bash
# Check if API is running
curl https://endlessperfect.com/tarot-api/docs

# Get your analytics
curl https://endlessperfect.com/tarot-api/api/analytics/user/@waqaas:endlessperfect.com/summary | jq

# Get your history
curl https://endlessperfect.com/tarot-api/api/readings/user/@waqaas:endlessperfect.com/history | jq
```

## 🏗️ Build & Run

### Build from Source
```bash
cd /home/waqaas/iambtarot/iamb-tarot
cargo build --release
```

### Run iamb
```bash
./target/release/iamb
```

### Configuration
- Config: `~/.config/iamb/config.toml`
- Sessions: `~/.local/share/iamb/profiles/`

## 📁 Important Files

### Local Development
- `cards.csv` - Ground truth for all card data
- `src/tarot_api.rs` - API client
- `src/commands.rs` - Command handlers
- `src/tarot_cards.rs` - Card data structures
- `src/base.rs` - Action definitions

### Server Files
- `~/matrix/tarot-api/models.py` - Database models
- `~/matrix/tarot-api/routes.py` - API endpoints
- `~/matrix/tarot-api/.env` - Configuration
- `~/matrix/docker-compose.yml` - Container setup

## 🐛 Troubleshooting

### API Not Responding
```bash
docker restart tarot-api
docker logs tarot-api --tail 50
```

### Database Issues
```bash
# Check database connection
docker exec matrix-postgres-1 psql -U tarot_user -d tarot_readings -c "\dt"

# Check card count
docker exec matrix-postgres-1 psql -U tarot_user -d tarot_readings -c "SELECT COUNT(*) FROM card_database"
```

### Build Errors
```bash
# Clean and rebuild
cargo clean
cargo build --release
```

### History Not Showing
- Check API is running: `curl https://endlessperfect.com/tarot-api/docs`
- Verify you have readings: `:tarot 3` then `:tarothistory`
- Check API logs: `docker logs tarot-api --tail 50`

## 🌐 URLs

- **API Base:** `https://endlessperfect.com/tarot-api/`
- **API Docs:** `https://endlessperfect.com/tarot-api/docs`
- **Matrix Server:** `https://endlessperfect.com`
- **Server IP:** `3.15.195.126`

## 📞 Database Credentials

- **Host:** postgres (docker network) / 3.15.195.126 (external)
- **Port:** 5432
- **Database:** tarot_readings
- **User:** tarot_user
- **Password:** tarot_secure_password_bea6b8ff05a98a3e

## 🎯 Quick Test Sequence

```bash
# 1. Build
cd /home/waqaas/iambtarot/iamb-tarot
cargo build --release

# 2. Run
./target/release/iamb

# 3. In iamb (in a room):
:tarot 3
:tarothistory
:tarothistory 1
:tarothistory suits
:tarothistory summary
```

All output should appear as formatted messages in the room! 🔮
