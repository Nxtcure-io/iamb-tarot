# Image Preview in Terminal

## Overview

iamb has built-in support for displaying images directly in the terminal. This works for tarot cards and any other images posted to Matrix rooms.

## Configuration

Image preview is configured in `~/.config/iamb/config.toml`:

```toml
[settings.tunables.image_preview]
size = { width = 80, height = 20 }
protocol = { type = "halfblocks" }
```

## Supported Protocols

### 1. Halfblocks (Universal)
**Best for**: Any terminal
**Quality**: Low-res but works everywhere

```toml
protocol = { type = "halfblocks" }
```

Uses Unicode half-block characters (▀▄) to display images. Works in any terminal but lower quality.

### 2. Sixel
**Best for**: xterm, mlterm, foot, WezTerm
**Quality**: High-res

```toml
protocol = { type = "sixel" }
```

High-quality image rendering. Check if your terminal supports sixels:
```bash
echo $TERM
# Should contain "sixel" or be xterm-compatible
```

### 3. Kitty Graphics Protocol
**Best for**: Kitty terminal
**Quality**: Highest

```toml
protocol = { type = "kitty" }
```

Best quality. Only works in Kitty terminal.

### 4. iTerm2 Protocol
**Best for**: iTerm2 (macOS)
**Quality**: High

```toml
protocol = { type = "iterm2" }
```

Only for iTerm2 on macOS.

## Size Configuration

Adjust the preview size (in terminal cells):

```toml
size = { width = 80, height = 20 }
```

- **width**: Number of columns (characters wide)
- **height**: Number of rows (lines tall)

### Recommended Sizes

**Small cards** (for chat):
```toml
size = { width = 40, height = 15 }
```

**Medium cards** (default):
```toml
size = { width = 80, height = 20 }
```

**Large cards** (full screen):
```toml
size = { width = 120, height = 40 }
```

**Tarot card aspect ratio** (2:3.33):
```toml
size = { width = 60, height = 100 }  # Tall cards
```

## Testing Image Preview

1. **Update config** with image preview settings
2. **Restart iamb**
3. **Post a tarot card**:
   ```
   :tarot fool
   ```
4. **Images should appear** inline in the scrollback

## Terminal Recommendations

### Best Experience

1. **Kitty** - Best overall
   ```bash
   sudo apt install kitty
   # In config.toml:
   protocol = { type = "kitty" }
   ```

2. **WezTerm** - Great cross-platform
   ```bash
   # Download from https://wezfurlong.org/wezterm/
   # In config.toml:
   protocol = { type = "sixel" }
   ```

3. **foot** - Lightweight (Wayland)
   ```bash
   sudo apt install foot
   # In config.toml:
   protocol = { type = "sixel" }
   ```

### Works Everywhere

**Halfblocks** - Use if you can't change terminals:
```toml
protocol = { type = "halfblocks" }
```

Quality is lower but works in any terminal including:
- gnome-terminal
- xterm
- konsole
- alacritty
- tmux/screen

## Troubleshooting

### Images don't appear

1. **Check config syntax**:
   ```bash
   cat ~/.config/iamb/config.toml
   ```

2. **Restart iamb** after config changes

3. **Try halfblocks** (most compatible):
   ```toml
   protocol = { type = "halfblocks" }
   ```

### Images are distorted

Adjust the size to match your card aspect ratio:
```toml
# Tarot cards are typically 2:3.33 ratio
size = { width = 30, height = 50 }
```

### Images are too small/large

Change the size values:
```toml
# Larger
size = { width = 100, height = 30 }

# Smaller
size = { width = 40, height = 12 }
```

### Wrong protocol for terminal

Check your terminal type:
```bash
echo $TERM
ps -p $$ -o comm=
```

Match protocol to terminal:
- `kitty` → `type = "kitty"`
- `xterm*` → `type = "sixel"` or `"halfblocks"`
- `iTerm.app` → `type = "iterm2"`
- Other → `type = "halfblocks"`

## Example Configurations

### For Kitty Terminal

```toml
[settings.tunables.image_preview]
size = { width = 80, height = 25 }
protocol = { type = "kitty" }
```

### For Standard Terminals (Universal)

```toml
[settings.tunables.image_preview]
size = { width = 60, height = 20 }
protocol = { type = "halfblocks" }
```

### For Sixel-Compatible Terminals

```toml
[settings.tunables.image_preview]
size = { width = 80, height = 25 }
protocol = { type = "sixel" }
```

### For Large Tarot Cards

```toml
[settings.tunables.image_preview]
size = { width = 45, height = 75 }  # 2:3.33 ratio
protocol = { type = "halfblocks" }
```

## Font Size Adjustment (Kitty/iTerm2)

For Kitty and iTerm2, you can specify font size:

```toml
protocol = { type = "kitty", font_size = [11, 24] }
# [width, height] in pixels
```

## Current Configuration

Your current config (`~/.config/iamb/config.toml`):

```toml
[settings.tunables.image_preview]
size = { width = 80, height = 20 }
protocol = { type = "halfblocks" }
```

This uses halfblocks (universal compatibility) with medium-sized previews.

## Testing Different Protocols

To test which protocol works best:

1. **Try halfblocks first** (guaranteed to work)
2. **If you have Kitty**, try `type = "kitty"`
3. **If you have sixel support**, try `type = "sixel"`
4. **Restart iamb** after each change
5. **Post a test card**: `:tarot fool`

## Performance Notes

- **Halfblocks**: Fast, low memory
- **Sixel**: Moderate speed, moderate memory
- **Kitty**: Fast, efficient
- **iTerm2**: Fast, efficient

Large images may take a moment to download and render.

## Disabling Image Preview

To disable image previews, remove or comment out the section:

```toml
# [settings.tunables.image_preview]
# size = { width = 80, height = 20 }
# protocol = { type = "halfblocks" }
```

Images will still upload but won't display inline.

## Next Steps

1. **Restart iamb** to load the new config
2. **Test with**: `:tarot fool`
3. **Adjust size** if needed
4. **Try different protocols** for better quality

Enjoy your tarot readings with inline card images! 🔮✨
