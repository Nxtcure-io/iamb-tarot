# Composite Tarot Spreads

## Overview

The `:tarot` command now supports numeric spreads that generate composite images with multiple cards arranged horizontally.

## Usage

### Numeric Spreads

```
:tarot 1    # Single card
:tarot 2    # Two cards side by side
:tarot 3    # Three cards in a row
:tarot 4    # Four cards in a row
:tarot 5    # Five cards in a row
:tarot 6    # Six cards (5 in first row, 1 in second)
:tarot 7    # Seven cards (5 + 2)
:tarot 8    # Eight cards (5 + 3)
:tarot 9    # Nine cards (5 + 4)
:tarot 10   # Ten cards (5 + 5)
```

### Legacy Support

```
:tarot threecard    # Still works, same as :tarot 3
```

### Single Cards

```
:tarot fool
:tarot six swords
:tarot science      # By title
```

## How It Works

### Layout Algorithm

- **Max 5 cards per row**
- **Multiple rows**: Cards wrap to second row after 5
- **Centered rows**: Each row is centered horizontally
- **Spacing**: 10px between cards, 20px padding around edges
- **Background**: Dark blue-gray (#14141E)

### Examples

**3 cards:**
```
┌─────────────────────────────────┐
│                                 │
│   [Card1]  [Card2]  [Card3]     │
│                                 │
└─────────────────────────────────┘
```

**6 cards:**
```
┌─────────────────────────────────────────────┐
│                                             │
│   [Card1]  [Card2]  [Card3]  [Card4]  [Card5] │
│                                             │
│              [Card6]                        │
│                                             │
└─────────────────────────────────────────────┘
```

**10 cards:**
```
┌─────────────────────────────────────────────┐
│                                             │
│   [Card1]  [Card2]  [Card3]  [Card4]  [Card5] │
│                                             │
│   [Card6]  [Card7]  [Card8]  [Card9]  [Card10]│
│                                             │
└─────────────────────────────────────────────┘
```

## Implementation

### New Module: `src/tarot_composite.rs`

**Key Functions:**
- `create_composite_spread()` - Generates composite image from card paths
- `save_composite_to_temp()` - Saves to temp file and returns path

**Features:**
- Automatic layout calculation
- Row centering
- PNG output format
- Temporary file management

### Updated Commands

**`iamb_tarot()` in `src/commands.rs`:**
- Detects numeric arguments
- Routes to `handle_n_card_spread()` for numbers 1-10
- Falls back to single card lookup for text

**`handle_n_card_spread()` (replaces `handle_three_card_spread`):**
- Selects N random cards
- Creates composite image
- Uploads to Matrix room

## Technical Details

### Image Processing

Uses the `image` crate to:
1. Load individual card images
2. Create blank canvas with calculated dimensions
3. Overlay cards at calculated positions
4. Encode to PNG format
5. Save to temporary file

### Random Selection

- Uses LCG (Linear Congruential Generator)
- Ensures unique cards (no duplicates in spread)
- Seed from current Unix timestamp

### Temporary Files

- Saved to system temp directory
- Named: `tarot_spread_{timestamp}.png`
- Location: `/tmp/tarot_spread_*.png` (Linux)

## Examples

### Quick Reading
```
:tarot 1
```
Single card for daily guidance.

### Past-Present-Future
```
:tarot 3
```
Classic three-card spread.

### Celtic Cross (simplified)
```
:tarot 10
```
Ten-card spread for comprehensive reading.

### Custom Spreads
```
:tarot 5    # Five-card horseshoe spread
:tarot 7    # Seven-card chakra spread
:tarot 4    # Four elements spread
```

## Advantages

### Over Sequential Posting

**Before (`:tarot threecard`):**
- Posted 3 separate images
- Posted 3 text labels
- Posted completion message
- Total: 7 messages

**Now (`:tarot 3`):**
- Posts 1 composite image
- All cards visible at once
- Cleaner chat history
- Total: 1 message

### Visual Benefits

- See entire spread at a glance
- Cards arranged in reading order
- Professional appearance
- Easy to screenshot for records

## File Structure

```
src/
├── tarot_composite.rs    # NEW: Composite image generation
├── tarot_cards.rs         # Card database and lookup
├── commands.rs            # UPDATED: Numeric spread support
└── main.rs                # UPDATED: Added composite module

/tmp/
└── tarot_spread_*.png     # Generated composite images
```

## Configuration

No configuration needed! Works out of the box with:
- Card images from `deck/` folder
- Card data from `cards.csv`
- Automatic layout based on card count

## Limitations

- **Max 10 cards**: Keeps images manageable
- **Max 5 per row**: Prevents images from being too wide
- **Same size cards**: Assumes all cards have same dimensions
- **PNG only**: Output format is always PNG

## Future Enhancements

Possible additions:

1. **Custom layouts**: Different arrangements (circle, cross, etc.)
2. **Card labels**: Add position labels on composite
3. **Reversed cards**: Rotate cards 180° for reversed meanings
4. **Spread templates**: Named spreads with specific layouts
5. **Card meanings**: Overlay interpretations on image
6. **Export options**: Save spreads to custom location

## Testing

### Test Single Card
```
:tarot fool
```

### Test Numeric Spreads
```
:tarot 1
:tarot 3
:tarot 5
:tarot 10
```

### Test Legacy
```
:tarot threecard
```

### Test Title Lookup
```
:tarot science      # Six of Swords
:tarot virtue       # Three of Wands
```

## Build Status

✅ Successfully compiled with `cargo build --release`

## Summary

The composite spread feature provides a professional, clean way to perform tarot readings in Matrix rooms. Instead of posting multiple separate images, users get a single composite image showing all cards in the spread, making readings easier to view and share.
