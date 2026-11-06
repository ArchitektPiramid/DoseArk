PRAGMA foreign_keys = ON;

CREATE TABLE brands (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  country TEXT,
  website TEXT,
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  CHECK (length(trim(name)) > 0)
);
CREATE INDEX idx_brands_name ON brands(name);

CREATE TABLE nutrients (
  id TEXT PRIMARY KEY,
  canonical_name TEXT NOT NULL UNIQUE,
  display_name TEXT NOT NULL,
  kind TEXT NOT NULL,
  default_unit TEXT NOT NULL,
  notes TEXT,
  reference_daily_amount REAL,
  reference_daily_unit TEXT,
  reference_source TEXT,
  reference_notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  CHECK (length(trim(canonical_name)) > 0),
  CHECK (length(trim(display_name)) > 0),
  CHECK (length(trim(default_unit)) > 0),
  CHECK (kind IN ('vitamin','mineral','amino_acid','fatty_acid','herb_extract','drug_substance','probiotic','other')),
  CHECK (reference_daily_amount IS NULL OR reference_daily_amount > 0),
  CHECK (reference_daily_amount IS NULL OR (reference_daily_unit IS NOT NULL AND length(trim(reference_daily_unit)) > 0))
);
CREATE INDEX idx_nutrients_display_name ON nutrients(display_name);
CREATE INDEX idx_nutrients_kind ON nutrients(kind);

CREATE TABLE products (
  id TEXT PRIMARY KEY,
  brand_id TEXT REFERENCES brands(id) ON DELETE SET NULL,
  name TEXT NOT NULL,
  category TEXT NOT NULL,
  dosage_form TEXT NOT NULL,
  serving_unit TEXT NOT NULL,
  serving_size_units REAL NOT NULL,
  unit_weight_mg REAL,
  barcode TEXT,
  description TEXT,
  archived INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  CHECK (length(trim(name)) > 0),
  CHECK (category IN ('supplement','medication','food','herb','other')),
  CHECK (dosage_form IN ('tablet','capsule','softgel','powder','liquid','drops','spray','gummy','other')),
  CHECK (length(trim(serving_unit)) > 0),
  CHECK (serving_size_units > 0),
  CHECK (unit_weight_mg IS NULL OR unit_weight_mg > 0),
  CHECK (archived IN (0,1))
);
CREATE INDEX idx_products_brand_id ON products(brand_id);
CREATE INDEX idx_products_name ON products(name);
CREATE INDEX idx_products_category ON products(category);
CREATE INDEX idx_products_archived ON products(archived);

CREATE TABLE product_nutrients (
  id TEXT PRIMARY KEY,
  product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
  nutrient_id TEXT NOT NULL REFERENCES nutrients(id) ON DELETE RESTRICT,
  amount_per_serving REAL NOT NULL,
  unit TEXT NOT NULL,
  label_daily_value_percent REAL,
  label_reference_amount REAL,
  label_reference_unit TEXT,
  source TEXT NOT NULL DEFAULT 'label',
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  UNIQUE (product_id, nutrient_id),
  CHECK (amount_per_serving > 0),
  CHECK (length(trim(unit)) > 0),
  CHECK (label_daily_value_percent IS NULL OR label_daily_value_percent >= 0),
  CHECK (label_reference_amount IS NULL OR label_reference_amount > 0),
  CHECK (label_reference_amount IS NULL OR (label_reference_unit IS NOT NULL AND length(trim(label_reference_unit)) > 0)),
  CHECK (source IN ('label','manual','calculated','imported','unknown'))
);
CREATE INDEX idx_product_nutrients_product_id ON product_nutrients(product_id);
CREATE INDEX idx_product_nutrients_nutrient_id ON product_nutrients(nutrient_id);
