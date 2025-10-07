#!/bin/bash

# Update docker-compose.yml to include Tarot API service
# Run this on the AWS Lightsail server

set -e

echo "=========================================="
echo "Updating Docker Compose Configuration"
echo "=========================================="
echo ""

COMPOSE_FILE="$HOME/matrix/docker-compose.yml"
BACKUP_FILE="$HOME/matrix/docker-compose.yml.backup.$(date +%Y%m%d_%H%M%S)"

# Backup existing docker-compose.yml
if [ -f "$COMPOSE_FILE" ]; then
    echo "Backing up existing docker-compose.yml..."
    cp "$COMPOSE_FILE" "$BACKUP_FILE"
    echo "✓ Backup saved to: $BACKUP_FILE"
else
    echo "Warning: $COMPOSE_FILE not found. Creating new file."
fi

# Add tarot-api service to docker-compose.yml
cat >> "$COMPOSE_FILE" << 'EOF'

  # Tarot API Service
  tarot-api:
    build: ./tarot-api
    container_name: tarot-api
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgresql://tarot_user:${TAROT_DB_PASSWORD}@postgres:5432/tarot_readings
      - MATRIX_SERVER=https://endlessperfect.com
      - API_SECRET_KEY=${TAROT_API_SECRET}
      - CORS_ORIGINS=*
    env_file:
      - ./tarot-api/.env
    depends_on:
      - postgres
    restart: unless-stopped
    networks:
      - matrix-network
EOF

echo "✓ Tarot API service added to docker-compose.yml"
echo ""

# Update nginx configuration
echo "Creating nginx configuration for Tarot API..."

NGINX_CONF="$HOME/matrix/nginx/tarot-api.conf"

cat > "$NGINX_CONF" << 'EOF'
# Tarot API reverse proxy configuration

location /tarot-api/ {
    proxy_pass http://tarot-api:8080/;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
    
    # CORS headers
    add_header 'Access-Control-Allow-Origin' '*' always;
    add_header 'Access-Control-Allow-Methods' 'GET, POST, PUT, DELETE, OPTIONS' always;
    add_header 'Access-Control-Allow-Headers' 'DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range,Authorization' always;
    
    # Handle preflight requests
    if ($request_method = 'OPTIONS') {
        add_header 'Access-Control-Allow-Origin' '*';
        add_header 'Access-Control-Allow-Methods' 'GET, POST, PUT, DELETE, OPTIONS';
        add_header 'Access-Control-Allow-Headers' 'DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range,Authorization';
        add_header 'Access-Control-Max-Age' 1728000;
        add_header 'Content-Type' 'text/plain; charset=utf-8';
        add_header 'Content-Length' 0;
        return 204;
    }
}
EOF

echo "✓ Nginx configuration created: $NGINX_CONF"
echo ""

# Instructions for including in main nginx config
echo "=========================================="
echo "Manual Steps Required"
echo "=========================================="
echo ""
echo "1. Include Tarot API config in main nginx configuration:"
echo "   Edit: $HOME/matrix/nginx/matrix.conf"
echo "   Add inside the server block:"
echo "   include /etc/nginx/conf.d/tarot-api.conf;"
echo ""
echo "2. Build and start the Tarot API service:"
echo "   cd $HOME/matrix"
echo "   docker compose build tarot-api"
echo "   docker compose up -d tarot-api"
echo ""
echo "3. Restart nginx:"
echo "   docker compose restart nginx"
echo ""
echo "4. Test the API:"
echo "   curl https://endlessperfect.com/tarot-api/health"
echo ""
echo "Backup of original docker-compose.yml: $BACKUP_FILE"
echo ""
