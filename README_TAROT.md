# iamb-tarot: Matrix Tarot Reading Client

A modified version of [iamb](https://iamb.chat) with built-in support for posting tarot card images to Matrix rooms.

## ✨ Features

- 📤 **Image Upload**: Post images to Matrix rooms using `:upload` command
- 🎴 **Tarot Command**: Convenient `:tarot` command for quick card posting
- 🔮 **Card Library**: Automatic card lookup from organized directory
- 🎨 **Sample Cards**: Includes script to generate 22 Major Arcana cards
- ⌨️ **Tab Completion**: Path completion for easy card selection

## 🚀 Quick Start

### 1. Build

```bash
cargo build --release
```

### 2. Setup Tarot Cards

```bash
./setup_tarot.sh
```

This creates sample cards in `~/.local/share/iamb/tarot_cards/`

### 3. Run

```bash
./target/release/iamb
```

### 4. Post a Card

In iamb:
```
:tarot fool
```

## 📖 Commands

| Command | Description | Example |
|---------|-------------|---------|
| `:upload <path>` | Upload any file | `:upload /tmp/card.png` |
| `:tarot <card>` | Post tarot card by name | `:tarot fool` |
| `:tarot <path>` | Post card from path | `:tarot ~/card.png` |
| `:tarot threecard` | Random 3-card spread | `:tarot threecard` |

## 📚 Documentation

- **[QUICK_START.md](QUICK_START.md)** - Get started in 5 minutes
- **[TAROT_USAGE.md](TAROT_USAGE.md)** - Complete usage guide
- **[TAROT_IMPLEMENTATION.md](TAROT_IMPLEMENTATION.md)** - Technical details
- **[MATRIX_SERVER_SUMMARY.md](MATRIX_SERVER_SUMMARY.md)** - Server configuration

## 🎴 Available Cards (after setup)

Major Arcana (22 cards):
- fool, magician, high-priestess, empress, emperor
- hierophant, lovers, chariot, strength, hermit
- wheel-of-fortune, justice, hanged-man, death, temperance
- devil, tower, star, moon, sun, judgement, world

## 🔧 Configuration

### Default Card Directory

```
~/.local/share/iamb/tarot_cards/
```

### Custom Directory

Set environment variable:

```bash
export TAROT_CARDS_DIR=/path/to/your/cards
```

## 💡 Example Usage

### Single Card Reading

```
:tarot fool
The Fool represents new beginnings and infinite potential.
```

### Three Card Spread (Automatic)

```
:tarot threecard
```

Automatically posts 3 random cards with Past/Present/Future labels!

### Three Card Spread (Manual)

```
:tarot empress
Past: Abundance and creativity

:tarot tower
Present: Sudden revelation

:tarot star
Future: Hope and inspiration
```

## 🌐 Matrix Server

This build is configured for:
- **Server**: https://endlessperfect.com
- **Users**: @waqaas, @keanu, @riva

See [MATRIX_SERVER_SUMMARY.md](MATRIX_SERVER_SUMMARY.md) for details.

## 🛠️ What's Modified

### Code Changes

1. **src/commands.rs**: Added `iamb_tarot()` function
2. **src/base.rs**: Added tarot to tab completion

### New Files

- `setup_tarot.sh` - Card generation script
- `TAROT_USAGE.md` - Usage documentation
- `QUICK_START.md` - Quick reference
- `TAROT_IMPLEMENTATION.md` - Technical docs

## 🧪 Testing

### Test Image Upload

```bash
# Create test image
python3 -c "
from PIL import Image, ImageDraw
img = Image.new('RGB', (300, 500), color='#2a1a4a')
draw = ImageDraw.Draw(img)
draw.rectangle([10, 10, 290, 490], outline='gold', width=3)
draw.text((150, 250), 'TEST', fill='gold', anchor='mm')
img.save('/tmp/test_card.png')
"

# Upload in iamb
:upload /tmp/test_card.png
```

### Test Tarot Command

```
:tarot fool
```

## 🔮 Use Cases

- **Personal Readings**: Daily card draws
- **Group Readings**: Collaborative spreads in shared rooms
- **Teaching**: Demonstrate card meanings
- **Practice**: Learn tarot with friends
- **Journaling**: Document readings in Matrix rooms

## 🚧 Future Ideas

- [ ] Random card selection
- [ ] Predefined spread templates
- [ ] Built-in card interpretations
- [ ] Reversed card support
- [ ] Batch card posting
- [ ] Card preview before sending

## 📝 License

Same as iamb: [Apache License, Version 2.0](LICENSE)

## 🙏 Credits

- **iamb**: https://github.com/ulyssa/iamb
- **Matrix**: https://matrix.org
- Built for tarot readings on Matrix

## 🐛 Troubleshooting

### Card not found?

```bash
ls ~/.local/share/iamb/tarot_cards/
```

### Can't see images?

Images may not display in all terminals but will be visible in:
- Element (web/desktop)
- Other Matrix clients
- Image-capable terminals (Kitty, iTerm2)

### Build fails?

```bash
cargo clean
cargo build --release
```

## 📞 Support

- Check documentation in this repo
- Original iamb docs: https://iamb.chat
- Matrix room: #iamb:0x.badd.cafe

---

**Happy Reading! 🔮✨**
