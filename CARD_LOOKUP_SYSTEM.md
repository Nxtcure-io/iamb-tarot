# Card Lookup System - Implementation Summary

## What Was Done

Refactored the `:tarot` command to use the CSV database and deck folder instead of the old simple directory lookup.

## New Components

### 1. **Tarot Cards Module** (`src/tarot_cards.rs`)

A complete card database system that:
- Loads all 78 cards from `cards.csv`
- Provides flexible card lookup by name
- Normalizes card names for easy searching
- Maps cards to their image files in the `deck/` folder

### 2. **Flexible Card Matching**

The `:tarot` command now accepts:

**Exact names:**
```
:tarot The Fool
:tarot Six of Swords
:tarot Knight of Cups
```

**Partial/flexible names:**
```
:tarot fool          → The Fool
:tarot six swords    → Six of Swords
:tarot knight cups   → Knight of Cups
:tarot priestess     → The Priestess
```

**Case insensitive:**
```
:tarot FOOL
:tarot ThE FoOl
:tarot the fool
```

**With or without "The":**
```
:tarot fool          → The Fool
:tarot the fool      → The Fool
:tarot star          → The Star
```

### 3. **Card Database Structure**

Each card has:
- `card`: Full card name (e.g., "The Fool")
- `image`: Image filename (e.g., "the_fool.jpg")
- `title`: Optional subtitle (e.g., "Science" for Six of Swords)
- Astrological attributes: planets, signs, elements
- Qabalistic attributes: path, sephira
- Suit information

## How It Works

### Card Lookup Flow

1. User types: `:tarot fool`
2. System normalizes input: `"fool"` → `"fool"` (lowercase, no spaces)
3. Searches card database for match
4. Finds "The Fool" → `the_fool.jpg`
5. Constructs path: `deck/the_fool.jpg`
6. Uploads image to Matrix room

### Normalization Rules

```rust
"The Fool"      → "fool"
"Six of Swords" → "sixofswords"
"Knight_of_Cups"→ "knightofcups"
```

- Converts to lowercase
- Removes "the " prefix
- Removes spaces, underscores, hyphens

### Three-Card Spread

Now uses the full 78-card database:
```
:tarot threecard
```

- Randomly selects 3 cards from all 78 cards
- Posts with Past/Present/Future labels
- Uses actual card images from deck folder

## Usage Examples

### Single Cards

```
:tarot fool
:tarot magician
:tarot six swords
:tarot knight cups
:tarot priestess
```

### Spreads

```
:tarot threecard
```

### Direct Paths (still supported)

```
:tarot /path/to/custom/card.jpg
:tarot ~/my-cards/special.png
```

## File Structure

```
iamb-tarot/
├── cards.csv              # Card database (78 cards)
├── deck/                  # Card images
│   ├── the_fool.jpg
│   ├── the_magician.jpg
│   ├── six_of_swords.jpg
│   └── ... (78 total)
├── src/
│   ├── tarot_cards.rs     # NEW: Card lookup module
│   ├── commands.rs        # UPDATED: Uses new lookup
│   └── main.rs            # UPDATED: Added module
└── Cargo.toml             # UPDATED: Added once_cell
