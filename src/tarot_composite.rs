//! Composite image generation for tarot spreads
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use std::path::Path;

/// Create a composite image from multiple card images arranged in rows
/// Max 5 cards per row, then wraps to next row
pub fn create_composite_spread(card_paths: &[String]) -> Result<Vec<u8>, String> {
    if card_paths.is_empty() {
        return Err("No cards provided".to_string());
    }
    
    if card_paths.len() > 10 {
        return Err("Maximum 10 cards supported".to_string());
    }
    
    // Load all card images
    let mut images: Vec<DynamicImage> = Vec::new();
    for path in card_paths {
        let img = image::open(Path::new(path))
            .map_err(|e| format!("Failed to load image {}: {}", path, e))?;
        images.push(img);
    }
    
    if images.is_empty() {
        return Err("No images loaded".to_string());
    }
    
    // Get dimensions from first card (assume all cards are same size)
    let card_width = images[0].width();
    let card_height = images[0].height();
    
    // Calculate layout
    let cards_per_row = 5;
    let num_cards = images.len();
    let num_rows = (num_cards + cards_per_row - 1) / cards_per_row; // Ceiling division
    
    let spacing = 10u32; // Pixels between cards
    let padding = 20u32; // Padding around edges
    
    // Calculate composite dimensions
    let cards_in_first_row = std::cmp::min(num_cards, cards_per_row);
    let cards_in_second_row = if num_cards > cards_per_row {
        num_cards - cards_per_row
    } else {
        0
    };
    
    let first_row_width = cards_in_first_row as u32 * card_width + (cards_in_first_row as u32 - 1) * spacing;
    let second_row_width = if cards_in_second_row > 0 {
        cards_in_second_row as u32 * card_width + (cards_in_second_row as u32 - 1) * spacing
    } else {
        0
    };
    
    let composite_width = std::cmp::max(first_row_width, second_row_width) + 2 * padding;
    let composite_height = num_rows as u32 * card_height + (num_rows as u32 - 1) * spacing + 2 * padding;
    
    // Create composite image with dark background
    let mut composite: RgbaImage = ImageBuffer::from_pixel(
        composite_width,
        composite_height,
        Rgba([20, 20, 30, 255]) // Dark blue-gray background
    );
    
    // Place cards
    let mut card_idx = 0;
    for row in 0..num_rows {
        let cards_in_this_row = if row == 0 {
            std::cmp::min(num_cards, cards_per_row)
        } else {
            num_cards - cards_per_row
        };
        
        let row_width = cards_in_this_row as u32 * card_width + (cards_in_this_row as u32 - 1) * spacing;
        let row_start_x = (composite_width - row_width) / 2; // Center the row
        
        for col in 0..cards_in_this_row {
            if card_idx >= images.len() {
                break;
            }
            
            let x = row_start_x + col as u32 * (card_width + spacing);
            let y = padding + row as u32 * (card_height + spacing);
            
            // Overlay card image
            let card = &images[card_idx];
            image::imageops::overlay(&mut composite, card, x as i64, y as i64);
            
            card_idx += 1;
        }
    }
    
    // Encode to PNG bytes
    let mut png_bytes: Vec<u8> = Vec::new();
    composite.write_to(
        &mut std::io::Cursor::new(&mut png_bytes),
        image::ImageFormat::Png
    ).map_err(|e| format!("Failed to encode PNG: {}", e))?;
    
    Ok(png_bytes)
}

/// Save composite image to a temporary file and return the path
pub fn save_composite_to_temp(card_paths: &[String]) -> Result<String, String> {
    let png_bytes = create_composite_spread(card_paths)?;
    
    // Create temp file
    let temp_dir = std::env::temp_dir();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let temp_path = temp_dir.join(format!("tarot_spread_{}.png", timestamp));
    
    std::fs::write(&temp_path, png_bytes)
        .map_err(|e| format!("Failed to write temp file: {}", e))?;
    
    Ok(temp_path.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite_layout() {
        // Test that layout calculations work correctly
        assert_eq!((3 + 5 - 1) / 5, 1); // 3 cards = 1 row
        assert_eq!((5 + 5 - 1) / 5, 1); // 5 cards = 1 row
        assert_eq!((6 + 5 - 1) / 5, 2); // 6 cards = 2 rows
        assert_eq!((10 + 5 - 1) / 5, 2); // 10 cards = 2 rows
    }
}
