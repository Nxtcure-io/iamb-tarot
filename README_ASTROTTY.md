# 🔮 AstroTTY - Terminal Tarot Reader

A feature-rich terminal-based tarot reading client with full reading history, analytics, and beautiful card displays. Built on the iamb Matrix client with tarot-specific enhancements.

## ✨ Features

- **🎴 Tarot Readings** - Draw multi-card spreads or look up individual cards
- **📊 Reading History** - Automatic saving of all your readings
- **📈 Analytics** - Track patterns in suits, elements, planets, signs, and sephira
- **🎨 Beautiful Display** - High-quality card images with Kitty/Sixel/halfblock support
- **🔒 Privacy** - Your readings are private and tied to your Matrix ID
- **⚡ Fast** - Terminal-native performance with Vim keybindings
- **🌐 Multi-user** - Each user has their own isolated reading history
- **📱 Matrix Integration** - Works in any Matrix room

## 🚀 Quick Start

### Automated Installation

#### Linux / macOS
```bash
git clone https://github.com/yourusername/iamb-tarot.git
cd iamb-tarot
./setup.sh
```

#### Windows
```cmd
git clone https://github.com/yourusername/iamb-tarot.git
cd iamb-tarot
setup.bat
```

The setup script will guide you through:
1. Installing Rust and dependencies
2. Creating your account
3. Building and installing AstroTTY
4. Configuring optimal image display

### Start Using AstroTTY

```bash
astrotty
```

Login with your credentials, then try:
```
:tarot 3              # Draw a 3-card spread
:tarot fool           # Look up "The Fool" card
:tarothistory         # View your reading history
:tarothistory suits   # See suit distribution graph
:tarothistory summary # View your analytics
```

## 📖 Commands

### Tarot Readings

| Command | Description |
|---------|-------------|
| `:tarot 3` | Draw a 3-card spread (auto-saved) |
| `:tarot 5` | Draw a 5-card spread (auto-saved) |
| `:tarot fool` | Look up "The Fool" card |
| `:tarot six swords` | Look up "Six of Swords" |
| `:tarot 3 info` | 3-card spread with meanings |

### History & Analytics

| Command | Description |
|---------|-------------|
| `:tarothistory` | List all your readings |
| `:tarothistory 1` | View reading #1 details |
| `:tarothistory 1 info` | View with card meanings |
| `:tarothistory suits` | Suit distribution graph |
| `:tarothistory elements` | Element distribution |
| `:tarothistory planets` | Planet distribution |
| `:tarothistory signs` | Zodiac sign distribution |
| `:tarothistory sephira` | Sephira distribution |
| `:tarothistory summary` | Overall statistics |

## 🎨 Terminal Support

### Excellent (High Quality)
- **Kitty** - GPU-accelerated, best quality
- **WezTerm** - Great sixel support
- **iTerm2** (macOS) - Native image protocol

### Good (Sixel)
- **Windows Terminal** - Sixel support
- **xterm** - Classic sixel
- **mlterm** - Sixel support
- **foot** - Wayland with sixel

### Basic (Universal)
- **Any terminal** - Unicode halfblocks

## 📊 Analytics Features

AstroTTY tracks and visualizes:

- **Suits** - Swords, Cups, Wands, Disks
- **Elements** - Fire, Water, Air, Earth
- **Planets** - Sol, Luna, Mars, Mercury, Jupiter, Venus, Saturn
- **Signs** - All 12 zodiac signs
- **Sephira** - Kether, Chokmah, Binah, Chesed, Geburah, Tiphareth, Netzach, Hod, Yesod, Malkuth

Example output:
```
SUIT Distribution (15 total)

Swords          ████████████████████ 40.0% (6)
Cups            ███████████ 26.7% (4)
Wands           ██████ 13.3% (2)
Disks           ████████ 20.0% (3)
```

## 🏗️ Architecture

- **Frontend** - Rust-based terminal client (iamb fork)
- **Backend** - FastAPI server with PostgreSQL
- **Server** - Matrix Synapse on AWS Lightsail
- **Database** - 78-card tarot deck with full attributes
- **API** - RESTful endpoints for readings and analytics

## 📁 Project Structure

```
iamb-tarot/
├── setup.sh              # Linux/macOS setup script
├── setup.bat             # Windows setup script
├── INSTALLATION.md       # Detailed installation guide
├── QUICK_REFERENCE.md    # Command reference
├── cards.csv             # Ground truth card database
├── src/
│   ├── tarot_api.rs      # API client
│   ├── tarot_cards.rs    # Card data structures
│   ├── tarot_composite.rs # Image composition
│   └── commands.rs       # Tarot commands
└── docs/                 # Documentation
```

## 🔧 Configuration

Config file: `~/.config/iamb/config.toml`

```toml
default_profile = "yourusername"

[profiles.yourusername]
user_id = "@yourusername:endlessperfect.com"
url = "https://endlessperfect.com"

[settings.image_preview]
protocol.type = "kitty"  # or "sixel" or "halfblocks"
```

## 🌐 Server Information

- **Matrix Server:** https://endlessperfect.com
- **API Endpoint:** https://endlessperfect.com/tarot-api/
- **API Docs:** https://endlessperfect.com/tarot-api/docs

## 🛠️ Development

### Build from Source

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/yourusername/iamb-tarot.git
cd iamb-tarot
cargo build --release

# Run
./target/release/iamb
```

### Run Tests

```bash
cargo test
```

### API Testing

```bash
# Check API status
curl https://endlessperfect.com/tarot-api/docs

# Get your analytics
curl https://endlessperfect.com/tarot-api/api/analytics/user/@yourusername:endlessperfect.com/summary
```

## 📚 Documentation

- **[INSTALLATION.md](INSTALLATION.md)** - Detailed installation instructions
- **[QUICK_REFERENCE.md](QUICK_REFERENCE.md)** - Command reference
- **[USER_ID_TRACKING_EXPLANATION.md](USER_ID_TRACKING_EXPLANATION.md)** - How the system works
- **[COMPLETE_FIX_SUMMARY.md](COMPLETE_FIX_SUMMARY.md)** - Technical details

## 🤝 Contributing

Contributions are welcome! This is a fork of [iamb](https://github.com/ulyssa/iamb) with tarot-specific features.

## 📜 License

Apache 2.0 - See LICENSE file for details

## 🙏 Credits

- **iamb** - Original Matrix client by ulyssa
- **Thoth Tarot** - Card imagery and system
- **Matrix** - Decentralized communication protocol

## 🔮 About the Tarot Deck

AstroTTY uses the Thoth Tarot deck, designed by Aleister Crowley and painted by Lady Frieda Harris. The deck includes:

- **22 Major Arcana** - The Fool through The Universe
- **56 Minor Arcana** - Four suits of 14 cards each
- **Full Attributions** - Planets, zodiac signs, elements, and Qabalistic correspondences

Each card includes:
- Traditional meanings
- Deep esoteric interpretations
- Astrological associations
- Elemental correspondences
- Qabalistic paths

## 💬 Community

- **Matrix Room:** #astrotty:endlessperfect.com
- **Issues:** GitHub Issues
- **Discussions:** GitHub Discussions

## 🎯 Roadmap

- [ ] Room-based reading filtering
- [ ] Shared readings between users
- [ ] Export reading history
- [ ] Custom spread templates
- [ ] Reversed card interpretations
- [ ] Daily card feature
- [ ] Reading journal notes

---

**Happy reading! 🔮✨**

*"The cards don't lie, but they do speak in riddles."*
