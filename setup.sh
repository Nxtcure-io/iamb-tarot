#!/bin/bash
# AstroTTY Setup Script for Linux/macOS
# This script sets up the AstroTTY tarot terminal client

set -e

BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}"
echo "╔═══════════════════════════════════════════════════════════╗"
echo "║                                                           ║"
echo "║              🔮 AstroTTY Setup Wizard 🔮                 ║"
echo "║                                                           ║"
echo "║         Terminal-based Tarot Reading Client              ║"
echo "║                                                           ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo -e "${NC}"
echo ""

# Check if running on supported OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     MACHINE=Linux;;
    Darwin*)    MACHINE=Mac;;
    *)          MACHINE="UNKNOWN:${OS}"
esac

if [ "$MACHINE" = "UNKNOWN:${OS}" ]; then
    echo -e "${RED}Error: Unsupported operating system: ${OS}${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Detected OS: ${MACHINE}${NC}"
echo ""

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check and install Rust
echo -e "${BLUE}[1/7] Checking Rust installation...${NC}"
if command_exists rustc; then
    RUST_VERSION=$(rustc --version)
    echo -e "${GREEN}✓ Rust is already installed: ${RUST_VERSION}${NC}"
else
    echo -e "${YELLOW}Rust not found. Installing Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${GREEN}✓ Rust installed successfully${NC}"
fi
echo ""

# Check for required dependencies
echo -e "${BLUE}[2/7] Checking system dependencies...${NC}"
MISSING_DEPS=()

if [ "$MACHINE" = "Linux" ]; then
    # Check for build essentials
    if ! command_exists gcc; then
        MISSING_DEPS+=("build-essential")
    fi
    if ! dpkg -l | grep -q libssl-dev; then
        MISSING_DEPS+=("libssl-dev")
    fi
    if ! dpkg -l | grep -q pkg-config; then
        MISSING_DEPS+=("pkg-config")
    fi
    
    if [ ${#MISSING_DEPS[@]} -gt 0 ]; then
        echo -e "${YELLOW}Missing dependencies: ${MISSING_DEPS[*]}${NC}"
        echo "Installing dependencies (requires sudo)..."
        sudo apt-get update
        sudo apt-get install -y "${MISSING_DEPS[@]}"
    fi
elif [ "$MACHINE" = "Mac" ]; then
    if ! command_exists brew; then
        echo -e "${YELLOW}Homebrew not found. Please install Homebrew first:${NC}"
        echo "  /bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
        exit 1
    fi
    
    if ! command_exists pkg-config; then
        brew install pkg-config
    fi
fi

echo -e "${GREEN}✓ All dependencies installed${NC}"
echo ""

# Get user credentials
echo -e "${BLUE}[3/7] Setting up Matrix account...${NC}"
echo ""
echo "Please enter your desired username (lowercase, no spaces):"
read -r USERNAME

# Validate username
if [[ ! "$USERNAME" =~ ^[a-z0-9_-]+$ ]]; then
    echo -e "${RED}Error: Username must contain only lowercase letters, numbers, hyphens, and underscores${NC}"
    exit 1
fi

MATRIX_ID="@${USERNAME}:endlessperfect.com"

# Check if username exists
echo "Checking username availability..."
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" "https://endlessperfect.com/tarot-api/api/users/${MATRIX_ID}")

if [ "$HTTP_CODE" = "200" ]; then
    echo -e "${YELLOW}Username '${USERNAME}' already exists in the tarot database.${NC}"
    echo "Do you want to use this existing account? (y/n)"
    read -r USE_EXISTING
    if [ "$USE_EXISTING" != "y" ]; then
        echo "Please run the setup again with a different username."
        exit 1
    fi
else
    echo -e "${GREEN}✓ Username '${USERNAME}' is available${NC}"
fi

echo ""
echo "Please enter your password:"
read -rs PASSWORD
echo ""
echo "Confirm password:"
read -rs PASSWORD_CONFIRM
echo ""

if [ "$PASSWORD" != "$PASSWORD_CONFIRM" ]; then
    echo -e "${RED}Error: Passwords do not match${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Credentials configured${NC}"
echo ""

# Build the application
echo -e "${BLUE}[4/7] Building AstroTTY (this may take a few minutes)...${NC}"
cargo build --release
echo -e "${GREEN}✓ Build complete${NC}"
echo ""

# Create config directory
echo -e "${BLUE}[5/7] Creating configuration...${NC}"
CONFIG_DIR="$HOME/.config/iamb"
mkdir -p "$CONFIG_DIR"

# Detect terminal capabilities for image protocol
IMAGE_PROTOCOL="halfblocks"
if [ -n "$KITTY_WINDOW_ID" ]; then
    IMAGE_PROTOCOL="kitty"
    echo -e "${GREEN}✓ Detected Kitty terminal - using Kitty graphics protocol${NC}"
elif [ "$TERM" = "xterm-kitty" ]; then
    IMAGE_PROTOCOL="kitty"
    echo -e "${GREEN}✓ Detected Kitty terminal - using Kitty graphics protocol${NC}"
elif command_exists mlterm || [ "$TERM" = "mlterm" ]; then
    IMAGE_PROTOCOL="sixel"
    echo -e "${GREEN}✓ Detected sixel support - using sixel protocol${NC}"
elif [ "$TERM_PROGRAM" = "WezTerm" ]; then
    IMAGE_PROTOCOL="sixel"
    echo -e "${GREEN}✓ Detected WezTerm - using sixel protocol${NC}"
else
    echo -e "${YELLOW}Using halfblocks for image display (works in any terminal)${NC}"
    echo "For better image quality, consider using Kitty, WezTerm, or a sixel-capable terminal"
fi

# Create config file
cat > "$CONFIG_DIR/config.toml" << EOF
# AstroTTY Configuration
default_profile = "${USERNAME}"

[profiles.${USERNAME}]
user_id = "${MATRIX_ID}"
url = "https://endlessperfect.com"

# General settings
[settings]
reaction_display = true
read_receipt_send = true
typing_notice_send = true

# Image preview settings
[settings.image_preview]
EOF

if [ "$IMAGE_PROTOCOL" = "kitty" ]; then
    cat >> "$CONFIG_DIR/config.toml" << EOF
protocol.type = "kitty"
protocol.kitty.scale = 2
protocol.kitty.resolution = "hdpi"
EOF
elif [ "$IMAGE_PROTOCOL" = "sixel" ]; then
    cat >> "$CONFIG_DIR/config.toml" << EOF
protocol.type = "sixel"
size = { width = 80, height = 20 }
EOF
else
    cat >> "$CONFIG_DIR/config.toml" << EOF
protocol.type = "halfblocks"
size = { width = 80, height = 20 }
EOF
fi

echo -e "${GREEN}✓ Configuration created at ${CONFIG_DIR}/config.toml${NC}"
echo ""

# Install binary
echo -e "${BLUE}[6/7] Installing AstroTTY...${NC}"
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"
cp target/release/iamb "$INSTALL_DIR/astrotty"
chmod +x "$INSTALL_DIR/astrotty"

# Add to PATH if not already there
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo -e "${YELLOW}Adding $INSTALL_DIR to PATH...${NC}"
    
    # Detect shell
    SHELL_NAME=$(basename "$SHELL")
    case "$SHELL_NAME" in
        bash)
            SHELL_RC="$HOME/.bashrc"
            ;;
        zsh)
            SHELL_RC="$HOME/.zshrc"
            ;;
        fish)
            SHELL_RC="$HOME/.config/fish/config.fish"
            ;;
        *)
            SHELL_RC="$HOME/.profile"
            ;;
    esac
    
    if [ "$SHELL_NAME" = "fish" ]; then
        echo "set -gx PATH $INSTALL_DIR \$PATH" >> "$SHELL_RC"
    else
        echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$SHELL_RC"
    fi
    
    export PATH="$INSTALL_DIR:$PATH"
    echo -e "${GREEN}✓ Added to PATH in ${SHELL_RC}${NC}"
    echo -e "${YELLOW}Note: Restart your terminal or run 'source ${SHELL_RC}' to use 'astrotty' command${NC}"
fi

echo -e "${GREEN}✓ AstroTTY installed to ${INSTALL_DIR}/astrotty${NC}"
echo ""

# Register with Matrix server and create tarot user
echo -e "${BLUE}[7/7] Registering with Matrix server...${NC}"

# Try to register on Matrix server
REGISTER_RESPONSE=$(curl -s -X POST "https://endlessperfect.com/_matrix/client/r0/register" \
    -H "Content-Type: application/json" \
    -d "{\"username\":\"${USERNAME}\",\"password\":\"${PASSWORD}\",\"auth\":{\"type\":\"m.login.dummy\"}}")

if echo "$REGISTER_RESPONSE" | grep -q "user_id"; then
    echo -e "${GREEN}✓ Matrix account created${NC}"
elif echo "$REGISTER_RESPONSE" | grep -q "User ID already taken"; then
    echo -e "${YELLOW}Matrix account already exists, will use existing account${NC}"
else
    echo -e "${YELLOW}Note: Matrix registration may require admin approval${NC}"
fi

# Create tarot database user
if [ "$HTTP_CODE" != "200" ]; then
    TAROT_RESPONSE=$(curl -s -X POST "https://endlessperfect.com/tarot-api/api/users" \
        -H "Content-Type: application/json" \
        -d "{\"matrix_id\":\"${MATRIX_ID}\",\"username\":\"${USERNAME}\"}")
    
    if echo "$TAROT_RESPONSE" | grep -q "user_id"; then
        echo -e "${GREEN}✓ Tarot database user created${NC}"
    fi
fi

echo ""
echo -e "${GREEN}"
echo "╔═══════════════════════════════════════════════════════════╗"
echo "║                                                           ║"
echo "║              ✨ Setup Complete! ✨                       ║"
echo "║                                                           ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo -e "${NC}"
echo ""
echo -e "${BLUE}Getting Started:${NC}"
echo ""
echo "1. Start AstroTTY:"
echo -e "   ${GREEN}astrotty${NC}"
echo ""
echo "2. Login with your credentials when prompted"
echo ""
echo "3. Try these commands in any room:"
echo -e "   ${GREEN}:tarot 3${NC}              - Draw a 3-card spread"
echo -e "   ${GREEN}:tarot fool${NC}           - Look up a specific card"
echo -e "   ${GREEN}:tarothistory${NC}         - View your reading history"
echo -e "   ${GREEN}:tarothistory suits${NC}   - See suit distribution"
echo -e "   ${GREEN}:tarothistory summary${NC} - View analytics"
echo ""
echo -e "${BLUE}Your Account:${NC}"
echo -e "   Username: ${GREEN}${USERNAME}${NC}"
echo -e "   Matrix ID: ${GREEN}${MATRIX_ID}${NC}"
echo -e "   Server: ${GREEN}https://endlessperfect.com${NC}"
echo ""
echo -e "${BLUE}Configuration:${NC}"
echo -e "   Config: ${GREEN}${CONFIG_DIR}/config.toml${NC}"
echo -e "   Binary: ${GREEN}${INSTALL_DIR}/astrotty${NC}"
echo -e "   Image Protocol: ${GREEN}${IMAGE_PROTOCOL}${NC}"
echo ""
echo -e "${YELLOW}Note: If 'astrotty' command is not found, restart your terminal${NC}"
echo ""
echo "For help and documentation, visit:"
echo "https://github.com/yourusername/iamb-tarot"
echo ""
echo "Happy reading! 🔮✨"
echo ""
