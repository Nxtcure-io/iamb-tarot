# Testing Image Preview

## Fixed Configuration

The config has been corrected. The issue was:
- ❌ Wrong: `[settings.tunables.image_preview]`
- ✅ Correct: `[settings.image_preview]`

## Current Config

Your `~/.config/iamb/config.toml` now has:

```toml
[settings]
reaction_display = true
read_receipt_send = true
typing_notice_send = true

[settings.image_preview]
protocol.type = "halfblocks"
size = { width = 80, height = 20 }
```

## Test Steps

1. **Restart iamb**:
   ```bash
   ./target/release/iamb
   ```

2. **Post a test card**:
   ```
   :tarot fool
   ```

3. **You should see**:
   - The image rendered with colored block characters
   - Not just "[Attached Image: fool.png]"
   - Actual visual representation of the card

## What Halfblocks Look Like

With halfblocks protocol, images appear as colored Unicode characters:
```
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
▀▀██████████▀▀▀▀
██░░░░░░░░░░██▀▀
██░░░░░░░░░░░░██
██░░THE FOOL░░██
██░░░░░░░░░░░░██
▀▀██████████▀▀▀▀
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
```

## If Still Not Working

### Check 1: Verify Config Loaded
Look for any error messages when iamb starts.

### Check 2: Try Larger Size
Edit config:
```toml
size = { width = 100, height = 30 }
```

### Check 3: Check Terminal Support
```bash
echo $TERM
# Should show: xterm-256color or similar
```

### Check 4: Alternative Protocols

**For better quality, try sixel** (if supported):
```toml
protocol.type = "sixel"
```

**For Kitty terminal**:
```toml
protocol.type = "kitty"
```

## Troubleshooting

### Images still show as "[Attached Image: ...]"

This means image preview is not enabled. Check:

1. **Config location**: `~/.config/iamb/config.toml`
2. **Syntax**: Must be `[settings.image_preview]`
3. **Restart**: Must restart iamb after config changes

### Images are garbled

Try different size:
```toml
size = { width = 60, height = 20 }
```

### Terminal doesn't support colors

Halfblocks requires 256-color support. Check:
```bash
tput colors
# Should output: 256
```

If not, try:
```bash
export TERM=xterm-256color
```

## Alternative: Use a Better Terminal

For best image preview experience:

### Install Kitty (Recommended)
```bash
sudo apt install kitty
kitty
# Then in config.toml:
protocol.type = "kitty"
```

### Install WezTerm
```bash
# Download from https://wezfurlong.org/wezterm/
# Then in config.toml:
protocol.type = "sixel"
```

## Expected Behavior

After fixing the config:

1. **Before**: `[Attached Image: fool.png]`
2. **After**: Actual colored block representation of the image

The image won't be photo-realistic with halfblocks, but you'll see:
- Colors and shapes
- Enough detail to identify the card
- Better than just text

## Quick Test

```bash
# 1. Restart iamb
./target/release/iamb

# 2. In iamb, type:
:tarot fool

# 3. Look for colored blocks instead of just text
```

## Success Indicators

✅ You'll know it's working when:
- You see colored Unicode characters (▀▄█░)
- The image area is larger than one line
- Colors match the card design
- You can make out the card's visual elements

❌ It's NOT working if:
- You only see: `[Attached Image: fool.png]`
- No colored blocks appear
- Image is just one line of text

## Next Steps

1. Restart iamb with the fixed config
2. Test with `:tarot fool`
3. If still not working, try `protocol.type = "sixel"`
4. Consider switching to Kitty terminal for best results
