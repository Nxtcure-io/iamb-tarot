#!/bin/bash

# Setup script for tarot card functionality in iamb
# This script creates the tarot cards directory and generates sample cards

set -e

TAROT_DIR="${TAROT_CARDS_DIR:-$HOME/.local/share/iamb/tarot_cards}"

echo "Setting up tarot cards directory..."
echo "Directory: $TAROT_DIR"

# Create directory
mkdir -p "$TAROT_DIR"

# Check if Python and PIL are available
if ! command -v python3 &> /dev/null; then
    echo "Error: python3 is required but not installed."
    exit 1
fi

# Create sample tarot cards
echo "Creating sample tarot card images..."

python3 << 'EOF'
import os
from PIL import Image, ImageDraw, ImageFont

# Major Arcana cards (0-21)
major_arcana = [
    ("fool", "0", "THE FOOL"),
    ("magician", "I", "THE MAGICIAN"),
    ("high-priestess", "II", "HIGH PRIESTESS"),
    ("empress", "III", "THE EMPRESS"),
    ("emperor", "IV", "THE EMPEROR"),
    ("hierophant", "V", "THE HIEROPHANT"),
    ("lovers", "VI", "THE LOVERS"),
    ("chariot", "VII", "THE CHARIOT"),
    ("strength", "VIII", "STRENGTH"),
    ("hermit", "IX", "THE HERMIT"),
    ("wheel-of-fortune", "X", "WHEEL OF FORTUNE"),
    ("justice", "XI", "JUSTICE"),
    ("hanged-man", "XII", "HANGED MAN"),
    ("death", "XIII", "DEATH"),
    ("temperance", "XIV", "TEMPERANCE"),
    ("devil", "XV", "THE DEVIL"),
    ("tower", "XVI", "THE TOWER"),
    ("star", "XVII", "THE STAR"),
    ("moon", "XVIII", "THE MOON"),
    ("sun", "XIX", "THE SUN"),
    ("judgement", "XX", "JUDGEMENT"),
    ("world", "XXI", "THE WORLD"),
]

# Card colors
colors = {
    'background': '#2a1a4a',
    'border': '#d4af37',
    'text': '#d4af37',
    'number': '#ffffff'
}

tarot_dir = os.environ.get('TAROT_CARDS_DIR', os.path.expanduser('~/.local/share/iamb/tarot_cards'))

for filename, number, title in major_arcana:
    img = Image.new('RGB', (300, 500), color=colors['background'])
    draw = ImageDraw.Draw(img)
    
    # Draw border
    draw.rectangle([10, 10, 290, 490], outline=colors['border'], width=3)
    
    # Draw inner decorative border
    draw.rectangle([20, 20, 280, 480], outline=colors['border'], width=1)
    
    # Draw title
    draw.text((150, 40), title, fill=colors['text'], anchor='mm')
    
    # Draw number in center
    draw.text((150, 250), number, fill=colors['number'], anchor='mm')
    
    # Draw bottom decoration
    draw.rectangle([50, 450, 250, 470], outline=colors['border'], width=1)
    
    # Save
    filepath = os.path.join(tarot_dir, f'{filename}.png')
    img.save(filepath)
    print(f'Created: {filepath}')

print(f'\nSuccessfully created {len(major_arcana)} tarot cards!')
print(f'Location: {tarot_dir}')
EOF

echo ""
echo "Setup complete!"
echo ""
echo "You can now use the :tarot command in iamb:"
echo "  :tarot fool"
echo "  :tarot magician"
echo "  :tarot high-priestess"
echo ""
echo "To use a custom directory, set TAROT_CARDS_DIR:"
echo "  export TAROT_CARDS_DIR=/path/to/your/cards"
echo ""
echo "List all available cards:"
echo "  ls -1 $TAROT_DIR | sed 's/\.png$//' | sort"
