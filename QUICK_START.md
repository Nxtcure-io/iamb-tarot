# Quick Start: Tarot Readings in iamb

## 1. Build the Modified iamb

```bash
cd /home/waqaas/iambtarot/iamb-tarot
cargo build --release
```

## 2. Set Up Tarot Cards

Run the setup script to create sample tarot card images:

```bash
./setup_tarot.sh
```

This creates 22 Major Arcana cards in `~/.local/share/iamb/tarot_cards/`

## 3. Start iamb

```bash
./target/release/iamb
```

Or install it:

```bash
cargo install --path .
iamb
```

## 4. Connect to Your Matrix Server

Your config is already set up at `~/.config/iamb/config.toml`:

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

## 5. Create or Join a Tarot Room

In iamb:

```
:create ++alias tarot-readings
```

Or join an existing room:

```
:join #tarot-readings:endlessperfect.com
```

## 6. Post Tarot Cards

### Single Card Reading

```
:tarot fool
The Fool represents new beginnings, innocence, and taking a leap of faith.
```

### Three Card Spread (Manual)

```
:tarot empress
Past: The Empress - Abundance and nurturing energy

:tarot tower
Present: The Tower - Sudden change and revelation

:tarot star
Future: The Star - Hope and renewed faith
```

### Three Card Spread (Automatic)

```
:tarot threecard
```

This will automatically:
1. Select 3 random cards from your collection
2. Post them with Past/Present/Future labels
3. Complete the spread in one command

### Using Direct Paths

```
:tarot /path/to/custom-card.png
```

## Available Commands

| Command | Description | Example |
|---------|-------------|---------|
| `:upload <path>` | Upload any file | `:upload /tmp/card.png` |
| `:tarot <card>` | Post a tarot card | `:tarot fool` |
| `:tarot <path>` | Post custom card | `:tarot ~/my-card.png` |
| `:tarot threecard` | Random 3-card spread | `:tarot threecard` |

## Available Tarot Cards

After running `setup_tarot.sh`, you'll have these cards:

- `fool` (0)
- `magician` (I)
- `high-priestess` (II)
- `empress` (III)
- `emperor` (IV)
- `hierophant` (V)
- `lovers` (VI)
- `chariot` (VII)
- `strength` (VIII)
- `hermit` (IX)
- `wheel-of-fortune` (X)
- `justice` (XI)
- `hanged-man` (XII)
- `death` (XIII)
- `temperance` (XIV)
- `devil` (XV)
- `tower` (XVI)
- `star` (XVII)
- `moon` (XVIII)
- `sun` (XIX)
- `judgement` (XX)
- `world` (XXI)

## Testing

### Test with the sample card we created:

```
:upload /tmp/test_tarot_card.png
```

### Test the tarot command:

```
:tarot fool
```

## Keyboard Shortcuts in iamb

- `i` - Enter insert mode (message bar)
- `Esc` - Exit insert mode
- `:` - Enter command mode
- `j/k` - Scroll up/down
- `G` - Go to bottom
- `gg` - Go to top

## Troubleshooting

### Card not found?

List available cards:

```bash
ls ~/.local/share/iamb/tarot_cards/
```

### Image not uploading?

Check file permissions:

```bash
ls -la ~/.local/share/iamb/tarot_cards/fool.png
```

Use direct upload:

```
:upload ~/.local/share/iamb/tarot_cards/fool.png
```

### Can't see images?

Images may not display in all terminals. They will still be uploaded and visible in:
- Element (web/desktop)
- Other Matrix clients
- Terminals with image support (Kitty, iTerm2, etc.)

## Next Steps

1. **Add your own cards**: Place PNG images in `~/.local/share/iamb/tarot_cards/`
2. **Create spreads**: Develop scripts to automate common spreads
3. **Add interpretations**: Create a database of card meanings
4. **Invite others**: Share the tarot room with other users

## Matrix Server Info

- **Server**: https://endlessperfect.com
- **Admin**: @keanu:endlessperfect.com
- **Users**: @waqaas, @keanu, @riva
- **Password**: Test123$ (for keanu, waqaas, riva)

## Resources

- Full documentation: `TAROT_USAGE.md`
- iamb website: https://iamb.chat
- Matrix server summary: `MATRIX_SERVER_SUMMARY.md`
