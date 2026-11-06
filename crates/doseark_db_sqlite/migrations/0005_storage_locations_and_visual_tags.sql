CREATE TABLE storage_locations (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  description TEXT,
  color TEXT,
  icon TEXT,
  sort_order INTEGER NOT NULL DEFAULT 0,
  is_archived INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  CHECK (length(trim(name)) > 0),
  CHECK (is_archived IN (0,1))
);

INSERT INTO storage_locations (id, name, description, color, icon, sort_order, is_archived, created_at, updated_at)
SELECT
  lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', (abs(random()) % 4) + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6))),
  trim(storage_location),
  NULL,
  '#6f91b8',
  'box',
  0,
  0,
  strftime('%Y-%m-%dT%H:%M:%fZ','now'),
  strftime('%Y-%m-%dT%H:%M:%fZ','now')
FROM inventory_lots
WHERE storage_location IS NOT NULL AND trim(storage_location) <> ''
GROUP BY trim(storage_location);

ALTER TABLE inventory_lots ADD COLUMN storage_location_id TEXT REFERENCES storage_locations(id);
UPDATE inventory_lots
SET storage_location_id = (
  SELECT id FROM storage_locations WHERE storage_locations.name = trim(inventory_lots.storage_location)
)
WHERE storage_location_id IS NULL AND storage_location IS NOT NULL AND trim(storage_location) <> '';

ALTER TABLE products ADD COLUMN ui_color TEXT;
ALTER TABLE products ADD COLUMN ui_icon TEXT;
ALTER TABLE products ADD COLUMN user_importance TEXT NOT NULL DEFAULT 'normal' CHECK (user_importance IN ('low','normal','high','critical'));

CREATE INDEX idx_inventory_lots_storage_location_id ON inventory_lots(storage_location_id);
CREATE INDEX idx_storage_locations_archived_sort ON storage_locations(is_archived, sort_order, name);
