# Tarot Card Image Posting in iamb

## Overview

This modified version of iamb includes functionality to post images (specifically tarot cards) to Matrix rooms. The implementation leverages iamb's existing upload capabilities and adds a convenient `:tarot` command for tarot readings.

## Image Upload Methods

### Method 1: Using `:upload` command (Built-in)

Upload any image file to the current room:

```
:upload /path/to/image.png
```

This works with any file type and automatically detects the MIME type.

### Method 2: Using `:tarot` command (New)

The `:tarot` command is specifically designed for tarot card readings:

```
:tarot fool
:tarot the-magician
:tarot /path/to/custom-card.png
```

#### How it works:

1. **Card name lookup**: If you provide just a card name (e.g., `fool`), it will look for the card in:
   - `$TAROT_CARDS_DIR/<card-name>.png` (if env var is set)
   - `~/.local/share/iamb/tarot_cards/<card-name>.png` (default)

2. **Direct path**: If you provide a full path (contains `/` or starts with `~`), it uploads that file directly.

## Setup for Tarot Readings

### 1. Create a tarot cards directory

```bash
mkdir -p ~/.local/share/iamb/tarot_cards
```

### 2. Add your tarot card images

Place your tarot card images in the directory with lowercase names:

```bash
~/.local/share/iamb/tarot_cards/
├── fool.png
├── magician.png
├── high-priestess.png
├── empress.png
└── ... (other cards)
```

### 3. (Optional) Set custom directory

You can use a custom directory by setting the `TAROT_CARDS_DIR` environment variable:

```bash
export TAROT_CARDS_DIR=/path/to/my/tarot/cards
```

Add this to your `~/.bashrc` or `~/.zshrc` to make it permanent.

## Example Tarot Reading Workflow

1. **Join or create a room** for tarot readings:
   ```
   :join #tarot-readings:endlessperfect.com
   ```

2. **Post a tarot card**:
   ```
   :tarot fool
   ```

3. **Add interpretation** (regular message):
   ```
   The Fool represents new beginnings and taking a leap of faith.
   ```

4. **Post multiple cards** for a spread:
   ```
   :tarot past
   :tarot present
   :tarot future
   ```

## Testing the Functionality

### Quick Test

1. **Create a test image**:
   ```bash
   python3 -c "
   from PIL import Image, ImageDraw
   img = Image.new('RGB', (300, 500), color='#2a1a4a')
   draw = ImageDraw.Draw(img)
   draw.rectangle([10, 10, 290, 490], outline='gold', width=3)
   draw.text((150, 250), 'TEST', fill='gold', anchor='mm')
   img.save('/tmp/test_card.png')
   "
   ```

2. **Upload it**:
   ```
   :upload /tmp/test_card.png
   ```

## Implementation Details

### Code Changes

1. **New command**: `iamb_tarot()` in `src/commands.rs`
   - Handles card name resolution
   - Supports both card names and direct paths
   - Provides helpful error messages

2. **Command registration**: Added to command list in `add_iamb_commands()`

3. **Tab completion**: Added `tarot` to path completion in `src/base.rs`

### How Images are Sent

The upload process (from `src/windows/room/chat.rs`):

1. File is read from disk
2. MIME type is detected automatically
3. File is sent using Matrix SDK's `room.send_attachment()`
4. A local echo message is displayed immediately
5. The actual message appears once the server confirms

### Supported Image Formats

- PNG (recommended for tarot cards)
- JPEG/JPG
- GIF
- WebP
- Any format supported by the `mime_guess` crate

## Matrix Server Configuration

Your Matrix server (`endlessperfect.com`) is already configured to handle media uploads:

- **Server**: https://endlessperfect.com
- **Users**: @waqaas, @keanu, @riva
- **Media uploads**: Enabled by default in Synapse

## Future Enhancements

Potential features to add:

1. **Random card selection**: `:tarot random` to pick a random card
2. **Spread templates**: `:tarot spread celtic-cross` for predefined spreads
3. **Card database**: Built-in card meanings and interpretations
4. **Reversed cards**: Support for reversed card images
5. **Batch upload**: `:tarot 3-card-spread` to upload multiple cards at once

## Troubleshooting

### "File not found" error

- Check that the card image exists in `~/.local/share/iamb/tarot_cards/`
- Verify the filename matches exactly (lowercase, with extension)
- Use `:upload /full/path/to/card.png` as a fallback

### Image doesn't display

- Some terminals don't support image previews
- The image is still uploaded and visible in other Matrix clients
- Try a terminal that supports sixels, Kitty, or iTerm2 protocols

### Permission denied

- Ensure you have read permissions on the image file
- Check that the tarot cards directory is accessible

## Building the Modified iamb

To rebuild after making changes:

```bash
cd /home/waqaas/iambtarot/iamb-tarot
cargo build --release
```

The binary will be at: `target/release/iamb`

## Resources

- **iamb documentation**: https://iamb.chat
- **Matrix SDK**: https://github.com/matrix-org/matrix-rust-sdk
- **Your Matrix server**: https://endlessperfect.com
