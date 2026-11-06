CREATE TABLE brands (
  id UUID PRIMARY KEY,

  name TEXT NOT NULL,
  country TEXT,
  website TEXT,
  notes TEXT,

  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,

  CONSTRAINT uq_brands_name UNIQUE (name),
  CONSTRAINT chk_brands_name_not_blank CHECK (length(trim(name)) > 0)
);

CREATE INDEX idx_brands_name
ON brands(name);

CREATE TABLE nutrients (
  id UUID PRIMARY KEY,

  canonical_name TEXT NOT NULL,
  display_name TEXT NOT NULL,
  kind TEXT NOT NULL,
  default_unit TEXT NOT NULL,
  notes TEXT,

  reference_daily_amount DOUBLE PRECISION,
  reference_daily_unit TEXT,
  reference_source TEXT,
  reference_notes TEXT,

  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,

  CONSTRAINT uq_nutrients_canonical_name UNIQUE (canonical_name),

  CONSTRAINT chk_nutrients_canonical_name_not_blank
    CHECK (length(trim(canonical_name)) > 0),

  CONSTRAINT chk_nutrients_display_name_not_blank
    CHECK (length(trim(display_name)) > 0),

  CONSTRAINT chk_nutrients_default_unit_not_blank
    CHECK (length(trim(default_unit)) > 0),

  CONSTRAINT chk_nutrients_kind
    CHECK (
      kind IN (
        'vitamin',
        'mineral',
        'amino_acid',
        'fatty_acid',
        'herb_extract',
        'drug_substance',
        'probiotic',
        'other'
      )
    ),

  CONSTRAINT chk_nutrients_reference_amount_positive
    CHECK (
      reference_daily_amount IS NULL
      OR reference_daily_amount > 0
    ),

  CONSTRAINT chk_nutrients_reference_unit_required_with_amount
    CHECK (
      reference_daily_amount IS NULL
      OR (
        reference_daily_unit IS NOT NULL
        AND length(trim(reference_daily_unit)) > 0
      )
    )
);

CREATE INDEX idx_nutrients_display_name
ON nutrients(display_name);

CREATE INDEX idx_nutrients_kind
ON nutrients(kind);

CREATE TABLE products (
  id UUID PRIMARY KEY,

  brand_id UUID REFERENCES brands(id) ON DELETE SET NULL,

  name TEXT NOT NULL,
  category TEXT NOT NULL,
  dosage_form TEXT NOT NULL,

  serving_unit TEXT NOT NULL,
  serving_size_units DOUBLE PRECISION NOT NULL,
  unit_weight_mg DOUBLE PRECISION,

  barcode TEXT,
  description TEXT,
  archived BOOLEAN NOT NULL DEFAULT FALSE,

  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,

  CONSTRAINT chk_products_name_not_blank
    CHECK (length(trim(name)) > 0),

  CONSTRAINT chk_products_category
    CHECK (
      category IN (
        'supplement',
        'medication',
        'food',
        'herb',
        'other'
      )
    ),

  CONSTRAINT chk_products_dosage_form
    CHECK (
      dosage_form IN (
        'tablet',
        'capsule',
        'softgel',
        'powder',
        'liquid',
        'drops',
        'spray',
        'gummy',
        'other'
      )
    ),

  CONSTRAINT chk_products_serving_unit_not_blank
    CHECK (length(trim(serving_unit)) > 0),

  CONSTRAINT chk_products_serving_size_positive
    CHECK (serving_size_units > 0),

  CONSTRAINT chk_products_unit_weight_positive
    CHECK (
      unit_weight_mg IS NULL
      OR unit_weight_mg > 0
    )
);

CREATE INDEX idx_products_brand_id
ON products(brand_id);

CREATE INDEX idx_products_name
ON products(name);

CREATE INDEX idx_products_category
ON products(category);

CREATE INDEX idx_products_archived
ON products(archived);

CREATE TABLE product_nutrients (
  id UUID PRIMARY KEY,

  product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
  nutrient_id UUID NOT NULL REFERENCES nutrients(id) ON DELETE RESTRICT,

  amount_per_serving DOUBLE PRECISION NOT NULL,
  unit TEXT NOT NULL,

  label_daily_value_percent DOUBLE PRECISION,
  label_reference_amount DOUBLE PRECISION,
  label_reference_unit TEXT,

  source TEXT NOT NULL DEFAULT 'label',
  notes TEXT,

  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,

  CONSTRAINT uq_product_nutrient UNIQUE (product_id, nutrient_id),

  CONSTRAINT chk_product_nutrients_amount_positive
    CHECK (amount_per_serving > 0),

  CONSTRAINT chk_product_nutrients_unit_not_blank
    CHECK (length(trim(unit)) > 0),

  CONSTRAINT chk_product_nutrients_label_dv_positive
    CHECK (
      label_daily_value_percent IS NULL
      OR label_daily_value_percent >= 0
    ),

  CONSTRAINT chk_product_nutrients_label_reference_positive
    CHECK (
      label_reference_amount IS NULL
      OR label_reference_amount > 0
    ),

  CONSTRAINT chk_product_nutrients_label_reference_unit_required
    CHECK (
      label_reference_amount IS NULL
      OR (
        label_reference_unit IS NOT NULL
        AND length(trim(label_reference_unit)) > 0
      )
    ),

  CONSTRAINT chk_product_nutrients_source
    CHECK (
      source IN (
        'label',
        'manual',
        'calculated',
        'imported',
        'unknown'
      )
    )
);

CREATE INDEX idx_product_nutrients_product_id
ON product_nutrients(product_id);

CREATE INDEX idx_product_nutrients_nutrient_id
ON product_nutrients(nutrient_id);
