//! Tarot card database and lookup functionality
use std::collections::HashMap;
use std::path::PathBuf;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct TarotCard {
    pub card: String,
    pub image: String,
    pub title: Option<String>,
    pub planet_orb: Option<String>,
    pub planet_house: Option<String>,
    pub sign_1: Option<String>,
    pub sign_2: Option<String>,
    pub sign_3: Option<String>,
    pub suit_1: Option<String>,
    pub suit_2: Option<String>,
    pub path: Option<String>,
    pub sephira: Option<String>,
    pub element_1: Option<String>,
    pub element_2: Option<String>,
    pub info: Option<String>,
    pub deepinfo: Option<String>,
}

impl TarotCard {
    /// Get the full path to the card image
    pub fn image_path(&self) -> PathBuf {
        get_deck_dir().join(&self.image)
    }

    /// Get a normalized search key for this card
    pub fn search_key(&self) -> String {
        normalize_card_name(&self.card)
    }

    /// Get display name (uses title if available, otherwise card name)
    pub fn display_name(&self) -> String {
        if let Some(title) = &self.title {
            if !title.is_empty() {
                return format!("{} ({})", self.card, title);
            }
        }
        self.card.clone()
    }
}

/// Normalize a card name for searching
/// Converts to lowercase, removes "the", removes underscores, removes spaces
pub fn normalize_card_name(name: &str) -> String {
    name.to_lowercase()
        .replace("the ", "")
        .replace("_", "")
        .replace(" ", "")
        .replace("-", "")
}

/// Get the deck directory path
pub fn get_deck_dir() -> PathBuf {
    // Try environment variable first
    if let Ok(deck_dir) = std::env::var("TAROT_DECK_DIR") {
        return PathBuf::from(deck_dir);
    }
    
    // Default to repo deck directory
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("deck")
}

/// Parse a CSV line handling quoted fields
fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                if in_quotes && chars.peek() == Some(&'"') {
                    // Escaped quote
                    current_field.push('"');
                    chars.next();
                } else {
                    // Toggle quote mode
                    in_quotes = !in_quotes;
                }
            },
            ',' if !in_quotes => {
                // Field separator
                fields.push(current_field.trim().to_string());
                current_field.clear();
            },
            _ => {
                current_field.push(c);
            }
        }
    }
    
    // Add last field
    fields.push(current_field.trim().to_string());
    fields
}

/// Load cards from CSV
fn load_cards_from_csv() -> HashMap<String, TarotCard> {
    let csv_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("cards.csv");
    let mut cards = HashMap::new();
    
    if let Ok(content) = std::fs::read_to_string(&csv_path) {
        let mut lines = content.lines();
        
        // Skip header
        lines.next();
        
        for line in lines {
            // Handle CSV with quoted fields (for info/deepinfo which may contain commas)
            let fields = parse_csv_line(line);
            if fields.len() >= 2 {
                let card = TarotCard {
                    card: fields[0].to_string(),
                    image: fields[1].to_string(),
                    title: fields.get(2).and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) }),
                    planet_orb: fields.get(3).and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) }),
                    planet_house: fields.get(4).and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) }),
                    sign_1: fields.get(5).and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) }),
                    sign_2: fields.get(6).and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) }),
                    sign_3: fields.get(7).and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) }),
                    suit_1: fields.get(8).and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) }),
                    suit_2: fields.get(9).and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) }),
                    path: fields.get(10).and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) }),
                    sephira: fields.get(11).and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) }),
                    element_1: fields.get(12).and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) }),
                    element_2: fields.get(13).and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) }),
                    info: fields.get(14).and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) }),
                    deepinfo: fields.get(15).and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) }),
                };
                
                // Index by normalized name
                let key = card.search_key();
                cards.insert(key, card);
            }
        }
    }
    
    cards
}

/// Global card database
static CARD_DATABASE: Lazy<HashMap<String, TarotCard>> = Lazy::new(load_cards_from_csv);

/// Find a card by flexible name matching (including by title)
pub fn find_card(query: &str) -> Option<&'static TarotCard> {
    let normalized_query = normalize_card_name(query);
    
    // Direct match by card name
    if let Some(card) = CARD_DATABASE.get(&normalized_query) {
        return Some(card);
    }
    
    // Try to match by title (e.g., "science" -> Six of Swords)
    if let Some(card) = CARD_DATABASE.values().find(|card| {
        if let Some(title) = &card.title {
            normalize_card_name(title) == normalized_query
        } else {
            false
        }
    }) {
        return Some(card);
    }
    
    // Partial match - find first card that contains the query in name or title
    CARD_DATABASE.values().find(|card| {
        let key = card.search_key();
        let name_match = key.contains(&normalized_query) || normalized_query.contains(&key);
        
        let title_match = if let Some(title) = &card.title {
            let normalized_title = normalize_card_name(title);
            normalized_title.contains(&normalized_query) || normalized_query.contains(&normalized_title)
        } else {
            false
        };
        
        name_match || title_match
    })
}

/// Get all cards
pub fn get_all_cards() -> Vec<&'static TarotCard> {
    CARD_DATABASE.values().collect()
}

/// Get cards by suit
pub fn get_cards_by_suit(suit: &str) -> Vec<&'static TarotCard> {
    let normalized_suit = suit.to_lowercase();
    CARD_DATABASE.values()
        .filter(|card| {
            card.suit_1.as_ref().map(|s| s.to_lowercase() == normalized_suit).unwrap_or(false) ||
            card.suit_2.as_ref().map(|s| s.to_lowercase() == normalized_suit).unwrap_or(false)
        })
        .collect()
}

/// Get major arcana cards
pub fn get_major_arcana() -> Vec<&'static TarotCard> {
    CARD_DATABASE.values()
        .filter(|card| card.suit_1.is_none() && card.suit_2.is_none())
        .collect()
}

/// Get minor arcana cards
pub fn get_minor_arcana() -> Vec<&'static TarotCard> {
    CARD_DATABASE.values()
        .filter(|card| card.suit_1.is_some() || card.suit_2.is_some())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_card_name() {
        assert_eq!(normalize_card_name("The Fool"), "fool");
        assert_eq!(normalize_card_name("Six of Swords"), "sixofswords");
        assert_eq!(normalize_card_name("Knight_of_Cups"), "knightofcups");
    }

    #[test]
    fn test_find_card() {
        assert!(find_card("fool").is_some());
        assert!(find_card("The Fool").is_some());
        assert!(find_card("six swords").is_some());
        assert!(find_card("6 swords").is_none()); // Number not in name
    }

    #[test]
    fn test_card_counts() {
        assert_eq!(CARD_DATABASE.len(), 78);
        assert_eq!(get_major_arcana().len(), 22);
        assert_eq!(get_minor_arcana().len(), 56);
    }
}
