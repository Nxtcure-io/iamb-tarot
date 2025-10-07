# Tarot Reading History & Analytics System

## Overview

A comprehensive system for tracking tarot readings with detailed analytics on card attributes (planets, zodiac signs, sephira, suits, elements).

## Features

### `:tarot history` Commands

```
:tarot history              # List all readings with cards
:tarot history 1            # Show reading #1 details
:tarot history 1 info       # Show reading #1 with card info
:tarot history suits        # Graph of suit frequencies
:tarot history sephira      # Graph of sephira frequencies
:tarot history planets      # Graph of planet frequencies
:tarot history signs        # Graph of zodiac sign frequencies
:tarot history elements     # Graph of element frequencies
:tarot history summary      # Overall statistics summary
```

## Database Schema

### New Table: `reading_attributes`

Stores aggregated counts of card attributes per reading:

```sql
CREATE TABLE reading_attributes (
    attribute_id SERIAL PRIMARY KEY,
    reading_id INTEGER REFERENCES readings(reading_id),
    attribute_type VARCHAR(50),  -- 'planet', 'sign', 'sephira', 'suit', 'element'
    attribute_value VARCHAR(100), -- 'Mars', 'Aquarius', 'Yesod', 'Swords', 'Fire'
    count INTEGER
);
```

### Example Data

For a 3-card reading with:
- Six of Swords (Science)
- The Tower
- Nine of Cups (Happiness)

Attributes stored:
```
reading_id=1, attribute_type='suit', attribute_value='Swords', count=1
reading_id=1, attribute_type='suit', attribute_value='Cups', count=1
reading_id=1, attribute_type='element', attribute_value='Air', count=1
reading_id=1, attribute_type='element', attribute_value='Water', count=1
reading_id=1, attribute_type='planet', attribute_value='Mercury', count=1
reading_id=1, attribute_type='planet', attribute_value='Mars', count=1
reading_id=1, attribute_type='planet', attribute_value='Jupiter', count=1
reading_id=1, attribute_type='planet', attribute_value='Luna', count=1
reading_id=1, attribute_type='sephira', attribute_value='Tiphareth', count=1
reading_id=1, attribute_type='sephira', attribute_value='Yesod', count=1
...
```

## API Endpoints

### History Endpoints

**Get User History**
```
GET /api/readings/user/@user:endlessperfect.com/history
```

Response:
```json
{
  "total_readings": 15,
  "readings": [
    {
      "reading_id": 15,
      "spread_type": "threecard",
      "reading_date": "2025-10-07T04:30:00",
      "card_count": 3,
      "cards": [
        {"position": 0, "card_name": "The Fool", "label": "Past"},
        {"position": 1, "card_name": "Six of Swords", "label": "Present"},
        {"position": 2, "card_name": "The Star", "label": "Future"}
      ],
      "notes": null
    },
    ...
  ]
}
```

**Get Reading Details**
```
GET /api/readings/15/details
```

Response includes card info/deepinfo if available.

### Analytics Endpoints

**Get Attribute Frequency**
```
GET /api/analytics/user/@user:endlessperfect.com/attributes/suits
```

Response:
```json
{
  "attribute_type": "suits",
  "total_count": 45,
  "frequencies": {
    "Swords": 12,
    "Cups": 10,
    "Wands": 15,
    "Disks": 8
  },
  "percentages": {
    "Swords": 26.67,
    "Cups": 22.22,
    "Wands": 33.33,
    "Disks": 17.78
  }
}
```

**Get Analytics Summary**
```
GET /api/analytics/user/@user:endlessperfect.com/summary
```

Response:
```json
{
  "total_readings": 15,
  "total_cards_drawn": 45,
  "spread_types": [
    {"type": "threecard", "count": 10},
    {"type": "single", "count": 5}
  ],
  "top_attributes": {
    "suit": [
      {"value": "Wands", "count": 15},
      {"value": "Swords", "count": 12},
      {"value": "Cups", "count": 10}
    ],
    "element": [
      {"value": "Fire", "count": 18},
      {"value": "Air", "count": 14},
      {"value": "Water", "count": 13}
    ],
    ...
  }
}
```

## iamb Client Implementation

### Command Structure

```rust
fn iamb_tarot_history(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    let args = desc.arg.strings()?;
    
    if args.is_empty() {
        // Show all readings list
        return show_history_list(ctx);
    }
    
    match args[0].as_str() {
        "suits" | "sephira" | "planets" | "signs" | "elements" => {
            // Show frequency graph
            return show_attribute_graph(args[0], ctx);
        },
        "summary" => {
            return show_analytics_summary(ctx);
        },
        num if num.parse::<usize>().is_ok() => {
            // Show specific reading
            let reading_num = num.parse::<usize>().unwrap();
            let show_info = args.contains(&"info".to_string());
            return show_reading_details(reading_num, show_info, ctx);
        },
        _ => {
            return Err(CommandError::InvalidArgument);
        }
    }
}
```

### Text-Based Graphs

For `:tarot history suits`:

```
Suit Distribution (45 total cards across 15 readings)

Wands    ████████████████████████████████████ 33.3% (15)
Swords   ████████████████████████████ 26.7% (12)
Cups     ██████████████████████ 22.2% (10)
Disks    ████████████████ 17.8% (8)
```

For `:tarot history sephira`:

```
Sephira Distribution (45 total cards across 15 readings)

Tiphareth ████████████████████████ 20.0% (9)
Yesod     ██████████████████ 15.6% (7)
Netzach   ████████████████ 13.3% (6)
Binah     ████████████ 11.1% (5)
Chesed    ██████████ 8.9% (4)
Hod       ████████ 6.7% (3)
Geburah   ██████ 4.4% (2)
...
```

## Workflow

### When User Does `:tarot 3`

1. **Select 3 random cards**
2. **Create composite image**
3. **Upload to Matrix**
4. **Call API**: `POST /api/readings`
   ```json
   {
     "matrix_id": "@user:endlessperfect.com",
     "spread_type": "3",
     "cards": [
       {"position": 0, "card_name": "The Fool"},
       {"position": 1, "card_name": "Six of Swords"},
       {"position": 2, "card_name": "The Star"}
     ]
   }
   ```
5. **API calculates attributes**:
   - Looks up each card in `card_database`
   - Counts planets, signs, sephira, suits, elements
   - Stores in `reading_attributes` table
6. **Returns reading_id**

### When User Does `:tarot history`

1. **Call API**: `GET /api/readings/user/@user:endlessperfect.com/history`
2. **Display formatted list**:
   ```
   Tarot Reading History (15 total readings)
   
   1. Oct 7, 2025 - 3-card spread
      The Fool, Six of Swords, The Star
   
   2. Oct 6, 2025 - 5-card spread
      The Tower, Nine of Cups, Knight of Wands, The Empress, Two of Disks
   
   3. Oct 5, 2025 - Single card
      The Magus
   
   ...
   
   Use :tarot history <number> to see details
   Use :tarot history suits/sephira/etc for analytics
   ```

### When User Does `:tarot history 1`

1. **Call API**: `GET /api/readings/1/details`
2. **Display formatted reading**:
   ```
   Reading #1 - Oct 7, 2025
   Spread: 3-card (Past/Present/Future)
   
   Card 1 (Past): The Fool
   Card 2 (Present): Six of Swords (Science)
   Card 3 (Future): The Star
   
   Attributes:
   Suits: Swords (1), Air (1)
   Elements: Air (2), Water (1)
   Planets: Mercury (1), Sol (1), Neptune (1), Saturn (1)
   Sephira: Tiphareth (1)
   Signs: Aquarius (2)
   ```

### When User Does `:tarot history suits`

1. **Call API**: `GET /api/analytics/user/@user:endlessperfect.com/attributes/suits`
2. **Generate ASCII bar graph**
3. **Display in chat**

## Implementation Steps

### Phase 1: Backend (Server)

1. ✅ Create SQL schema (`04-add-analytics-schema.sql`)
2. ✅ Add `ReadingAttribute` model to `models.py`
3. ✅ Add analytics routes to `routes.py`
4. ✅ Update `POST /api/readings` to calculate attributes
5. Deploy updated API

### Phase 2: iamb Client

1. Add API client module (`src/api_client.rs`)
   - HTTP client setup
   - Functions for each endpoint
   
2. Add `:tarot history` command (`src/commands.rs`)
   - Parse subcommands
   - Route to appropriate handler
   
3. Add history display functions
   - `show_history_list()` - Format reading list
   - `show_reading_details()` - Format single reading
   - `show_attribute_graph()` - Generate ASCII graphs
   - `show_analytics_summary()` - Format summary stats

4. Add configuration
   ```toml
   [settings.tarot]
   api_url = "https://endlessperfect.com/tarot-api"
   auto_save = true
   ```

### Phase 3: Integration

1. Update `:tarot` commands to auto-save readings
2. Test all history commands
3. Add error handling for offline/API failures

## ASCII Graph Generation

```rust
fn generate_bar_graph(data: HashMap<String, f64>, max_width: usize) -> String {
    let max_value = data.values().cloned().fold(0.0, f64::max);
    let mut lines = Vec::new();
    
    for (label, value) in data.iter().sorted_by_key(|(_, v)| -v) {
        let bar_width = ((value / max_value) * max_width as f64) as usize;
        let bar = "█".repeat(bar_width);
        let percentage = (value / total * 100.0);
        lines.push(format!("{:<12} {} {:.1}% ({})", label, bar, percentage, value));
    }
    
    lines.join("\n")
}
```

## Benefits

1. **Track Progress**: See how your readings evolve over time
2. **Pattern Recognition**: Identify recurring themes (suits, elements, etc.)
3. **Learning Tool**: Understand which archetypes appear most
4. **Statistical Insights**: Data-driven tarot practice
5. **Historical Record**: Never lose a reading
6. **Shareable**: Export or share reading history

## Example Session

```
> :tarot 3
[Composite image posted]

> :tarot history
Tarot Reading History (1 total reading)
1. Oct 7, 2025 - 3-card spread
   The Fool, Six of Swords, The Star

> :tarot 5 info
[Composite image + info text posted]

> :tarot history
Tarot Reading History (2 total readings)
1. Oct 7, 2025 - 5-card spread
   ...
2. Oct 7, 2025 - 3-card spread
   ...

> :tarot history suits
Suit Distribution (8 total cards across 2 readings)
Swords   ████████████████ 37.5% (3)
Wands    ████████████ 25.0% (2)
Cups     ████████ 12.5% (1)
Disks    ████████ 12.5% (1)
(Major)  ████ 12.5% (1)

> :tarot history 1 info
Reading #1 - Oct 7, 2025
...
[Full details with card info]
```

## Future Enhancements

- Export readings to JSON/CSV
- Compare readings side-by-side
- Time-series analysis (trends over weeks/months)
- Predictive analytics (which cards appear together)
- Reading journal with notes
- Share readings with other users
- Import readings from other sources
