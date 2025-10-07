# AstroTTY Installation Guide

## Quick Start

AstroTTY is a terminal-based tarot reading client with full reading history and analytics.

### Automated Setup (Recommended)

#### Linux / macOS
```bash
./setup.sh
```

#### Windows
```cmd
setup.bat
```

The setup script will:
1. ✅ Check and install Rust if needed
2. ✅ Install system dependencies
3. ✅ Prompt for username and password
4. ✅ Check username availability
5. ✅ Build the application
6. ✅ Create configuration with optimal image display settings
7. ✅ Install as `astrotty` command
8. ✅ Register your account with the Matrix server
9. ✅ Create your tarot database user

### After Installation

Start AstroTTY:
```bash
astrotty
```

Login with your credentials when prompted, then try:
```
:tarot 3              # Draw a 3-card spread
:tarot fool           # Look up a specific card
:tarothistory         # View your reading history
:tarothistory suits   # See suit distribution
:tarothistory summary # View analytics
```

---

## Manual Installation

If you prefer to install manually or the automated script doesn't work:

### Prerequisites

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install build-essential libssl-dev pkg-config curl
```

#### Linux (Fedora/RHEL)
```bash
sudo dnf install gcc openssl-devel pkg-config curl
```

#### macOS
```bash
# Install Homebrew if not already installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install pkg-config
```

#### Windows
1. Install [Rust](https://rustup.rs/)
2. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)

### Install Rust

If you don't have Rust installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Build AstroTTY

```bash
# Clone the repository
git clone https://github.com/yourusername/iamb-tarot.git
cd iamb-tarot

# Build the application
cargo build --release

# Install the binary
mkdir -p ~/.local/bin
cp target/release/iamb ~/.local/bin/astrotty
chmod +x ~/.local/bin/astrotty

# Add to PATH (add this to your ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.local/bin:$PATH"
```

### Create Configuration

Create `~/.config/iamb/config.toml`:

```toml
default_profile = "yourusername"

[profiles.yourusername]
user_id = "@yourusername:endlessperfect.com"
url = "https://endlessperfect.com"

[settings]
reaction_display = true
read_receipt_send = true
typing_notice_send = true

[settings.image_preview]
# Choose one based on your terminal:
# For Kitty terminal:
protocol.type = "kitty"
protocol.kitty.scale = 2
protocol.kitty.resolution = "hdpi"

# For sixel-capable terminals (xterm, mlterm, wezterm, foot):
# protocol.type = "sixel"
# size = { width = 80, height = 20 }

# For any terminal (fallback):
# protocol.type = "halfblocks"
# size = { width = 80, height = 20 }
```

### Register Your Account

#### Option 1: Via API (Automatic)
```bash
# Register Matrix account (if registration is open)
curl -X POST "https://endlessperfect.com/_matrix/client/r0/register" \
  -H "Content-Type: application/json" \
  -d '{"username":"yourusername","password":"yourpassword","auth":{"type":"m.login.dummy"}}'

# Create tarot database user
curl -X POST "https://endlessperfect.com/tarot-api/api/users" \
  -H "Content-Type: application/json" \
  -d '{"matrix_id":"@yourusername:endlessperfect.com","username":"yourusername"}'
```

#### Option 2: Contact Admin
If registration is disabled, contact the server administrator to create an account for you.

---

## Terminal Recommendations

For the best image display quality:

### Excellent Support
- **Kitty** - Best quality, GPU-accelerated
- **WezTerm** - Great sixel support
- **iTerm2** (macOS) - Native image protocol

### Good Support
- **Windows Terminal** - Sixel support
- **xterm** - Classic sixel support
- **mlterm** - Sixel support
- **foot** - Wayland terminal with sixel

### Basic Support
- **Any terminal** - Falls back to Unicode halfblocks

---

## Configuration Options

### Image Display Protocols

#### Kitty Protocol (Best Quality)
```toml
[settings.image_preview]
protocol.type = "kitty"
protocol.kitty.scale = 2
protocol.kitty.resolution = "hdpi"
```

#### Sixel Protocol (Good Quality)
```toml
[settings.image_preview]
protocol.type = "sixel"
size = { width = 80, height = 20 }
```

#### Halfblocks (Universal)
```toml
[settings.image_preview]
protocol.type = "halfblocks"
size = { width = 80, height = 20 }
```

### Multiple Profiles

You can configure multiple accounts:

```toml
default_profile = "main"

[profiles.main]
user_id = "@alice:endlessperfect.com"
url = "https://endlessperfect.com"

[profiles.alt]
user_id = "@bob:endlessperfect.com"
url = "https://endlessperfect.com"
```

Switch profiles by changing `default_profile` or using the `-P` flag:
```bash
astrotty -P alt
```

---

## Troubleshooting

### "astrotty: command not found"
Add `~/.local/bin` to your PATH:
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Build Errors
Make sure you have all dependencies installed:
```bash
# Ubuntu/Debian
sudo apt-get install build-essential libssl-dev pkg-config

# Fedora/RHEL
sudo dnf install gcc openssl-devel pkg-config
```

### Images Not Displaying
1. Check your terminal supports the configured protocol
2. Try changing to `halfblocks` in config
3. Verify image preview is enabled in settings

### Connection Errors
1. Check your internet connection
2. Verify the server is running: `curl https://endlessperfect.com/_matrix/client/versions`
3. Check your credentials are correct

### Username Already Exists
If the username is taken, either:
1. Choose a different username
2. If it's your account, use the existing credentials

---

## Uninstallation

```bash
# Remove binary
rm ~/.local/bin/astrotty

# Remove configuration (optional)
rm -rf ~/.config/iamb

# Remove session data (optional)
rm -rf ~/.local/share/iamb
```

---

## Getting Help

- **Documentation:** See `QUICK_REFERENCE.md` for command reference
- **Issues:** Report bugs on GitHub
- **Server Status:** Check `https://endlessperfect.com/_matrix/client/versions`

---

## What's Installed

After installation, you'll have:

- **Binary:** `~/.local/bin/astrotty` (or `%USERPROFILE%\.local\bin\astrotty.exe` on Windows)
- **Config:** `~/.config/iamb/config.toml`
- **Sessions:** `~/.local/share/iamb/profiles/`
- **Command:** `astrotty` available in your terminal

---

## Next Steps

1. Start AstroTTY: `astrotty`
2. Login with your credentials
3. Join or create a room
4. Try your first reading: `:tarot 3`
5. View your history: `:tarothistory`
6. Explore analytics: `:tarothistory summary`

Happy reading! 🔮✨
