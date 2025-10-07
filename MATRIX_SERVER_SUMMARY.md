# Matrix Synapse Server Setup Summary

## Server Infrastructure

**AWS Lightsail Instance:**
- IP Address: `3.15.195.126`
- Username: `ubuntu`
- SSH Key: `/home/waqaas/antler.pem`
- OS: Ubuntu 24.04 LTS
- Domain: `endlessperfect.com` (DNS via Cloudflare)

**Server Location on Instance:**
- Installation directory: `~/matrix/`
- All services run via Docker Compose

## Services Running

**Docker Containers:**
1. **matrix-synapse-1** - Matrix homeserver
   - Internal port: 8008
   - Image: `matrixdotorg/synapse:latest`
   
2. **matrix-postgres-1** - PostgreSQL database
   - Port: 5432 (internal only)
   - Image: `postgres:15`
   - Database: `synapse`
   - User: `synapse`
   - Password: `synapse_password_change_me`

3. **matrix-nginx-1** - Reverse proxy with SSL
   - Ports: 80 (HTTP), 443 (HTTPS), 8448 (Federation)
   - Image: `nginx:alpine`

4. **matrix-certbot-1** - SSL certificate management
   - Auto-renewal every 12 hours
   - Certificates valid until: January 2026

## Server Configuration

**Synapse Config:** `~/matrix/synapse/homeserver.yaml`
- Server name: `endlessperfect.com`
- Database: PostgreSQL (not SQLite)
- Registration: Enabled
- Registration without verification: Enabled
- Federation: Enabled on port 8448

**Key Files:**
- Docker Compose: `~/matrix/docker-compose.yml`
- Nginx Config: `~/matrix/nginx/matrix.conf`
- SSL Certificates: `~/matrix/nginx/certbot/conf/live/endlessperfect.com/`
- Signing Key: `~/matrix/synapse/endlessperfect.com.signing.key`
- Log Config: `~/matrix/synapse/endlessperfect.com.log.config`

## User Accounts

| Username | Matrix ID | Password | Admin |
|----------|-----------|----------|-------|
| admin | @admin:endlessperfect.com | SecurePassword123! | Yes |
| keanu | @keanu:endlessperfect.com | Test123$ | Yes |
| waqaas | @waqaas:endlessperfect.com | Test123$ | No |
| riva | @riva:endlessperfect.com | Test123$ | No |

**Note:** Matrix usernames are case-insensitive and stored as lowercase.

## Network & Firewall

**AWS Lightsail Firewall Rules (Required):**
- TCP Port 80 - HTTP (Let's Encrypt verification)
- TCP Port 443 - HTTPS (Matrix client API)
- TCP Port 8448 - Matrix Federation (server-to-server)

**Cloudflare Configuration:**
- DNS A record: `endlessperfect.com` → `3.15.195.126`
- SSL/TLS mode: Full (strict) recommended
- Proxy status: Can be enabled (orange cloud)

## API Endpoints

**Well-Known Endpoints:**
- Server: `https://endlessperfect.com/.well-known/matrix/server`
  - Returns: `{"m.server": "endlessperfect.com:8448"}`
- Client: `https://endlessperfect.com/.well-known/matrix/client`
  - Returns: `{"m.homeserver": {"base_url": "https://endlessperfect.com"}}`

**Matrix API:**
- Client API: `https://endlessperfect.com/_matrix/client/`
- Federation API: `https://endlessperfect.com:8448/_matrix/federation/`
- Versions endpoint: `https://endlessperfect.com/_matrix/client/versions`

## Server Management Commands

**SSH Access:**
```bash
ssh -i /home/waqaas/antler.pem ubuntu@3.15.195.126
```

**Docker Compose Commands:**
```bash
cd ~/matrix

# View status
docker compose ps

# View logs
docker compose logs -f synapse
docker compose logs -f nginx

# Restart services
docker compose restart

# Stop all services
docker compose down

# Start all services
docker compose up -d

# Restart specific service
docker compose restart synapse
```

**Create New User:**
```bash
docker exec matrix-synapse-1 register_new_matrix_user \
  -u <username> -p <password> --no-admin \
  -c /data/homeserver.yaml http://localhost:8008

# For admin user, replace --no-admin with -a
```

**Make User Admin (via PostgreSQL):**
```bash
docker exec matrix-postgres-1 psql -U synapse -d synapse -c \
  "UPDATE users SET admin = 1 WHERE name = '@username:endlessperfect.com';"
```

**View Users:**
```bash
docker exec matrix-postgres-1 psql -U synapse -d synapse -c \
  "SELECT name, admin FROM users;"
```

## Client Configuration (iamb)

**Local Config File:** `~/.config/iamb/config.toml`

**Example Configuration:**
```toml
default_profile = "waqaas"

[profiles.waqaas]
user_id = "@waqaas:endlessperfect.com"
url = "https://endlessperfect.com"

[profiles.keanu]
user_id = "@keanu:endlessperfect.com"
url = "https://endlessperfect.com"

[profiles.riva]
user_id = "@riva:endlessperfect.com"
url = "https://endlessperfect.com"
```

**Session Data Location:** `~/.local/share/iamb/profiles/<profile_name>/`

**Delete Session:**
```bash
rm -rf ~/.local/share/iamb/profiles/<profile_name>
```

## Testing & Verification

**Test Server Connectivity:**
```bash
# Test well-known endpoints
curl https://endlessperfect.com/.well-known/matrix/server
curl https://endlessperfect.com/.well-known/matrix/client

# Test Matrix API
curl https://endlessperfect.com/_matrix/client/versions

# Test login
curl -X POST https://endlessperfect.com/_matrix/client/r0/login \
  -H "Content-Type: application/json" \
  -d '{"type":"m.login.password","user":"waqaas","password":"Test123$"}'
```

**Federation Testing:**
- Use: https://federationtester.matrix.org/
- Enter domain: `endlessperfect.com`

## Security Notes

1. **Database Password:** Currently `synapse_password_change_me` - consider changing
2. **Registration:** Open registration is enabled - consider disabling after creating accounts
3. **Admin Password:** Change `SecurePassword123!` after first login
4. **Backups:** Important directories to backup:
   - `~/matrix/postgres/` (database)
   - `~/matrix/synapse/` (config and keys)
   - `~/matrix/nginx/certbot/conf/` (SSL certificates)

## Common Issues & Solutions

**M_FORBIDDEN Error:**
- Check username is lowercase (e.g., `keanu` not `Keanu`)
- Verify password is correct
- Ensure account exists in database

**Connection Timeout:**
- Check AWS Lightsail firewall rules
- Verify Cloudflare SSL/TLS mode is "Full (strict)"
- Check nginx is running: `docker compose ps`

**SSL Certificate Issues:**
- Certificates auto-renew via certbot
- Manual renewal: `docker compose restart certbot`

## Important Secrets

**Registration Shared Secret:** `Ac^&FLmRNrLGOu4~UiUVe#Od@X8l0j^l^IY8Q:Hod~uSQ-_,BB`
**Macaroon Secret:** `=6mJDT:mvNqxzw*vwqj*rV+;WV5~9YzeDsqIi9dykXpwR4j#5i`
**Form Secret:** `dD7mMTIAOK=FQ^q.QpR_&uqsDGO6BvriI3TNSmCwt@V*ADflab`

## Resources

- Matrix Synapse Documentation: https://matrix-org.github.io/synapse/
- Matrix Specification: https://spec.matrix.org/
- iamb Client: https://iamb.chat
- Element Web Client: https://element.io/

## Setup Date

Server configured: October 6, 2025
SSL Certificate expires: January 5, 2026
