CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS storage_locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    name TEXT NOT NULL UNIQUE,
    description TEXT,

    color TEXT,
    icon TEXT,

    sort_order INTEGER NOT NULL DEFAULT 0,
    is_archived BOOLEAN NOT NULL DEFAULT FALSE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

ALTER TABLE inventory_lots
ADD COLUMN IF NOT EXISTS storage_location_id UUID REFERENCES storage_locations(id);

-- Migrate old text locations into normalized locations.
INSERT INTO storage_locations (
    name,
    description,
    color,
    icon,
    sort_order,
    is_archived,
    created_at,
    updated_at
)
SELECT DISTINCT
    TRIM(storage_location) AS name,
    NULL AS description,
    '#6f91b8' AS color,
    'box' AS icon,
    0 AS sort_order,
    FALSE AS is_archived,
    now() AS created_at,
    now() AS updated_at
FROM inventory_lots
WHERE storage_location IS NOT NULL
  AND TRIM(storage_location) <> ''
ON CONFLICT (name) DO NOTHING;

UPDATE inventory_lots il
SET storage_location_id = sl.id
FROM storage_locations sl
WHERE il.storage_location_id IS NULL
  AND il.storage_location IS NOT NULL
  AND TRIM(il.storage_location) = sl.name;

-- Inventory statuses.
ALTER TABLE inventory_lots
DROP CONSTRAINT IF EXISTS chk_inventory_lots_status;

ALTER TABLE inventory_lots
ADD CONSTRAINT chk_inventory_lots_status
CHECK (
    status IN (
        'active',
        'reserve',
        'on_hold',
        'empty',
        'expired',
        'discarded',
        'archived'
    )
);

-- Product visual customization.
ALTER TABLE products
ADD COLUMN IF NOT EXISTS ui_color TEXT,
ADD COLUMN IF NOT EXISTS ui_icon TEXT,
ADD COLUMN IF NOT EXISTS user_importance TEXT NOT NULL DEFAULT 'normal';

ALTER TABLE products
DROP CONSTRAINT IF EXISTS chk_products_user_importance;

ALTER TABLE products
ADD CONSTRAINT chk_products_user_importance
CHECK (
    user_importance IN (
        'low',
        'normal',
        'high',
        'critical'
    )
);

CREATE INDEX IF NOT EXISTS idx_inventory_lots_storage_location_id
ON inventory_lots(storage_location_id);

CREATE INDEX IF NOT EXISTS idx_inventory_lots_status
ON inventory_lots(status);

CREATE INDEX IF NOT EXISTS idx_storage_locations_archived_sort
ON storage_locations(is_archived, sort_order, name);
