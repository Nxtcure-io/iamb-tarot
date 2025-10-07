# Three Card Spread Feature

## Overview

Added automatic three-card tarot spread functionality to iamb. Users can now type `:tarot threecard` to get a complete Past/Present/Future reading with randomly selected cards.

## What Was Added

### 1. New SendAction Variant

**File**: `src/base.rs`

Added `TarotSpread(Vec<(String, String)>)` to the `SendAction` enum:
- Takes a vector of (label, card_path) tuples
- Allows posting multiple cards with text labels in sequence

### 2. Spread Handler

**File**: `src/windows/room/chat.rs`

Added handler for `SendAction::TarotSpread`:
- Iterates through cards in the spread
- Posts label as text message
- Uploads card image
- Sends completion message

### 3. Three Card Spread Function

**File**: `src/commands.rs`

Added `handle_three_card_spread()` function:
- Reads available cards from tarot directory
- Uses simple LCG random number generator
- Selects 3 unique random cards
- Creates spread with Past/Present/Future labels
- Returns `SendAction::TarotSpread`

### 4. Command Integration

**File**: `src/commands.rs`

Modified `iamb_tarot()` to detect spread keywords:
- Checks for "threecard" or "three-card"
- Routes to spread handler
- Falls back to single card upload

## Usage

```
:tarot threecard
```

Or:

```
:tarot three-card
```

## How It Works

1. **User types** `:tarot threecard`
2. **Command parser** detects spread keyword
3. **Handler reads** all PNG files from tarot directory
4. **Random selection** picks 3 unique cards
5. **Spread creation** builds vector with labels:
   - 🔮 **Past** + card path
   - 🔮 **Present** + card path
   - 🔮 **Future** + card path
6. **Upload sequence**:
   - Post "🔮 **Past**"
   - Upload first card image
   - Post "🔮 **Present**"
   - Upload second card image
   - Post "🔮 **Future**"
   - Upload third card image
   - Post "[Tarot Spread Complete]"

## Example Output

In the Matrix room, users will see:

```
🔮 **Past**
[Image: hermit.png]

🔮 **Present**
[Image: tower.png]

🔮 **Future**
[Image: star.png]

[Tarot Spread Complete]
```

## Random Number Generation

Uses a simple Linear Congruential Generator (LCG):
- Seed: Current Unix timestamp
- Formula: `rng = rng * 1103515245 + 12345`
- Ensures 3 unique cards are selected
- Good enough for tarot readings (not cryptographic)

## Error Handling

- **Directory not found**: Returns error with directory path
- **Not enough cards**: Requires at least 3 cards, suggests running `setup_tarot.sh`
- **File read errors**: Handled by upload logic

## Future Enhancements

Potential additions:

1. **More spread types**:
   - `:tarot celtic-cross` (10 cards)
   - `:tarot horseshoe` (7 cards)
   - `:tarot relationship` (5 cards)

2. **Custom spreads**:
   - `:tarot spread <name> <count>` with custom labels

3. **Reversed cards**:
   - Random chance for reversed orientation
   - Flip image or add "Reversed" label

4. **Card meanings**:
   - Built-in interpretations
   - Post meaning with each card

5. **Spread templates**:
   - User-defined spread configurations
   - Save/load custom spreads

## Technical Details

### Data Flow

```
:tarot threecard
    ↓
iamb_tarot() detects "threecard"
    ↓
handle_three_card_spread()
    ↓
Read tarot directory
    ↓
Select 3 random cards
    ↓
Create Vec<(label, path)>
    ↓
SendAction::TarotSpread
    ↓
send_command() in chat.rs
    ↓
For each (label, path):
  - Send label as text
  - Upload card image
    ↓
Send completion message
```

### Performance

- Directory read: O(n) where n = number of files
- Random selection: O(3) iterations (constant)
- Upload: 3 images + 4 text messages
- Total time: ~1-3 seconds depending on network

### Dependencies

No new dependencies added. Uses:
- `std::fs` - Directory reading
- `std::time` - Timestamp for RNG seed
- `std::collections::HashSet` - Unique card selection
- Existing upload infrastructure

## Testing

### Manual Test

1. Build: `cargo build --release`
2. Run: `./target/release/iamb`
3. Join a room
4. Type: `:tarot threecard`
5. Verify: 3 cards posted with labels

### Expected Behavior

- ✅ 3 different cards selected
- ✅ Labels appear before each card
- ✅ Cards upload successfully
- ✅ Completion message appears
- ✅ No duplicate cards in spread

## Code Quality

- ✅ Compiles without errors
- ✅ No warnings
- ✅ Follows existing code patterns
- ✅ Proper error handling
- ✅ Documentation updated

## Documentation Updates

Updated files:
- `QUICK_START.md` - Added threecard command
- `README_TAROT.md` - Added automatic spread example
- `THREECARD_FEATURE.md` - This file

## Build Status

✅ Successfully compiled with `cargo build --release`

## Conclusion

The three-card spread feature provides a convenient way to perform tarot readings in Matrix rooms. Users can now get a complete Past/Present/Future reading with a single command, making the tarot functionality more practical and engaging.
