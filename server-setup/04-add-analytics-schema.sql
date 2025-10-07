-- Add analytics tables for tracking card attributes in readings

-- Table to store attribute counts per reading
CREATE TABLE IF NOT EXISTS reading_attributes (
    attribute_id SERIAL PRIMARY KEY,
    reading_id INTEGER REFERENCES readings(reading_id) ON DELETE CASCADE,
    attribute_type VARCHAR(50) NOT NULL,  -- 'planet', 'sign', 'sephira', 'suit', 'element'
    attribute_value VARCHAR(100) NOT NULL,
    count INTEGER NOT NULL DEFAULT 1,
    UNIQUE(reading_id, attribute_type, attribute_value)
);

-- Index for fast queries
CREATE INDEX IF NOT EXISTS idx_reading_attributes_reading ON reading_attributes(reading_id);
CREATE INDEX IF NOT EXISTS idx_reading_attributes_type ON reading_attributes(attribute_type);
CREATE INDEX IF NOT EXISTS idx_reading_attributes_value ON reading_attributes(attribute_value);

-- View for aggregated statistics across all readings for a user
CREATE OR REPLACE VIEW user_attribute_stats AS
SELECT 
    r.user_id,
    ra.attribute_type,
    ra.attribute_value,
    SUM(ra.count) as total_count,
    COUNT(DISTINCT r.reading_id) as readings_with_attribute
FROM reading_attributes ra
JOIN readings r ON ra.reading_id = r.reading_id
GROUP BY r.user_id, ra.attribute_type, ra.attribute_value;

-- View for reading summaries
CREATE OR REPLACE VIEW reading_summaries AS
SELECT 
    r.reading_id,
    r.user_id,
    r.spread_type,
    r.reading_date,
    r.notes,
    COUNT(cd.card_id) as card_count,
    STRING_AGG(cd.card_name, ', ' ORDER BY cd.position) as cards
FROM readings r
LEFT JOIN cards_drawn cd ON r.reading_id = cd.reading_id
GROUP BY r.reading_id, r.user_id, r.spread_type, r.reading_date, r.notes;

COMMENT ON TABLE reading_attributes IS 'Stores aggregated counts of card attributes (planets, signs, sephira, suits, elements) per reading';
COMMENT ON VIEW user_attribute_stats IS 'Aggregated statistics of attributes across all readings for each user';
COMMENT ON VIEW reading_summaries IS 'Summary view of readings with card lists';
