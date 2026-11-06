CREATE TABLE inventory_lots (
  id UUID PRIMARY KEY,

  product_id UUID NOT NULL REFERENCES products(id) ON DELETE RESTRICT,

  lot_number TEXT,
  storage_location TEXT,

  units_initial DOUBLE PRECISION NOT NULL,
  units_remaining DOUBLE PRECISION NOT NULL,

  purchase_date DATE,
  opened_date DATE,
  expires_at DATE,

  price_amount DOUBLE PRECISION,
  price_currency TEXT,

  status TEXT NOT NULL DEFAULT 'active',
  notes TEXT,

  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,

  CONSTRAINT chk_inventory_lots_units_initial_positive
    CHECK (units_initial > 0),

  CONSTRAINT chk_inventory_lots_units_remaining_non_negative
    CHECK (units_remaining >= 0),

  CONSTRAINT chk_inventory_lots_units_remaining_not_above_initial
    CHECK (units_remaining <= units_initial),

  CONSTRAINT chk_inventory_lots_price_positive
    CHECK (
      price_amount IS NULL
      OR price_amount >= 0
    ),

  CONSTRAINT chk_inventory_lots_status
    CHECK (
      status IN (
        'active',
        'empty',
        'expired',
        'discarded',
        'archived'
      )
    )
);

CREATE INDEX idx_inventory_lots_product_id
ON inventory_lots(product_id);

CREATE INDEX idx_inventory_lots_status
ON inventory_lots(status);

CREATE INDEX idx_inventory_lots_expires_at
ON inventory_lots(expires_at);

CREATE TABLE dose_logs (
  id UUID PRIMARY KEY,

  product_id UUID NOT NULL REFERENCES products(id) ON DELETE RESTRICT,
  inventory_lot_id UUID REFERENCES inventory_lots(id) ON DELETE SET NULL,

  taken_at TIMESTAMPTZ NOT NULL,

  units_taken DOUBLE PRECISION NOT NULL,
  unit TEXT NOT NULL,

  serving_multiplier DOUBLE PRECISION NOT NULL DEFAULT 1,

  reason TEXT,
  notes TEXT,

  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,

  CONSTRAINT chk_dose_logs_units_taken_positive
    CHECK (units_taken > 0),

  CONSTRAINT chk_dose_logs_unit_not_blank
    CHECK (length(trim(unit)) > 0),

  CONSTRAINT chk_dose_logs_serving_multiplier_positive
    CHECK (serving_multiplier > 0)
);

CREATE INDEX idx_dose_logs_product_id
ON dose_logs(product_id);

CREATE INDEX idx_dose_logs_inventory_lot_id
ON dose_logs(inventory_lot_id);

CREATE INDEX idx_dose_logs_taken_at
ON dose_logs(taken_at);

CREATE TABLE stock_movements (
  id UUID PRIMARY KEY,

  inventory_lot_id UUID NOT NULL REFERENCES inventory_lots(id) ON DELETE CASCADE,
  product_id UUID NOT NULL REFERENCES products(id) ON DELETE RESTRICT,
  dose_log_id UUID REFERENCES dose_logs(id) ON DELETE SET NULL,

  occurred_at TIMESTAMPTZ NOT NULL,

  movement_type TEXT NOT NULL,
  quantity_delta DOUBLE PRECISION NOT NULL,
  quantity_after DOUBLE PRECISION NOT NULL,

  reason TEXT,
  notes TEXT,

  created_at TIMESTAMPTZ NOT NULL,

  CONSTRAINT chk_stock_movements_quantity_delta_not_zero
    CHECK (quantity_delta <> 0),

  CONSTRAINT chk_stock_movements_quantity_after_non_negative
    CHECK (quantity_after >= 0),

  CONSTRAINT chk_stock_movements_type
    CHECK (
      movement_type IN (
        'dose',
        'manual_add',
        'manual_remove',
        'correction',
        'discard',
        'open_package',
        'reconcile'
      )
    )
);

CREATE INDEX idx_stock_movements_inventory_lot_id
ON stock_movements(inventory_lot_id);

CREATE INDEX idx_stock_movements_product_id
ON stock_movements(product_id);

CREATE INDEX idx_stock_movements_dose_log_id
ON stock_movements(dose_log_id);

CREATE INDEX idx_stock_movements_occurred_at
ON stock_movements(occurred_at);

CREATE TABLE dose_log_nutrients (
  id UUID PRIMARY KEY,

  dose_log_id UUID NOT NULL REFERENCES dose_logs(id) ON DELETE CASCADE,
  nutrient_id UUID NOT NULL REFERENCES nutrients(id) ON DELETE RESTRICT,

  amount DOUBLE PRECISION NOT NULL,
  unit TEXT NOT NULL,

  reference_daily_amount DOUBLE PRECISION,
  reference_daily_unit TEXT,
  reference_source TEXT,

  percent_of_reference DOUBLE PRECISION,

  created_at TIMESTAMPTZ NOT NULL,

  CONSTRAINT uq_dose_log_nutrient UNIQUE (dose_log_id, nutrient_id),

  CONSTRAINT chk_dose_log_nutrients_amount_positive
    CHECK (amount > 0),

  CONSTRAINT chk_dose_log_nutrients_unit_not_blank
    CHECK (length(trim(unit)) > 0),

  CONSTRAINT chk_dose_log_nutrients_reference_amount_positive
    CHECK (
      reference_daily_amount IS NULL
      OR reference_daily_amount > 0
    )
);

CREATE INDEX idx_dose_log_nutrients_dose_log_id
ON dose_log_nutrients(dose_log_id);

CREATE INDEX idx_dose_log_nutrients_nutrient_id
ON dose_log_nutrients(nutrient_id);
