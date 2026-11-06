CREATE TABLE inventory_lots (
  id TEXT PRIMARY KEY,
  product_id TEXT NOT NULL REFERENCES products(id) ON DELETE RESTRICT,
  lot_number TEXT,
  storage_location TEXT,
  units_initial REAL NOT NULL,
  units_remaining REAL NOT NULL,
  purchase_date TEXT,
  opened_date TEXT,
  expires_at TEXT,
  price_amount REAL,
  price_currency TEXT,
  status TEXT NOT NULL DEFAULT 'active',
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  CHECK (units_initial > 0),
  CHECK (units_remaining >= 0),
  CHECK (units_remaining <= units_initial),
  CHECK (price_amount IS NULL OR price_amount >= 0),
  CHECK (status IN ('active','reserve','on_hold','empty','expired','discarded','archived'))
);
CREATE INDEX idx_inventory_lots_product_id ON inventory_lots(product_id);
CREATE INDEX idx_inventory_lots_status ON inventory_lots(status);
CREATE INDEX idx_inventory_lots_expires_at ON inventory_lots(expires_at);

CREATE TABLE dose_logs (
  id TEXT PRIMARY KEY,
  product_id TEXT NOT NULL REFERENCES products(id) ON DELETE RESTRICT,
  inventory_lot_id TEXT REFERENCES inventory_lots(id) ON DELETE SET NULL,
  taken_at TEXT NOT NULL,
  units_taken REAL NOT NULL,
  unit TEXT NOT NULL,
  serving_multiplier REAL NOT NULL DEFAULT 1,
  reason TEXT,
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  CHECK (units_taken > 0),
  CHECK (length(trim(unit)) > 0),
  CHECK (serving_multiplier > 0)
);
CREATE INDEX idx_dose_logs_product_id ON dose_logs(product_id);
CREATE INDEX idx_dose_logs_inventory_lot_id ON dose_logs(inventory_lot_id);
CREATE INDEX idx_dose_logs_taken_at ON dose_logs(taken_at);

CREATE TABLE stock_movements (
  id TEXT PRIMARY KEY,
  inventory_lot_id TEXT NOT NULL REFERENCES inventory_lots(id) ON DELETE CASCADE,
  product_id TEXT NOT NULL REFERENCES products(id) ON DELETE RESTRICT,
  dose_log_id TEXT REFERENCES dose_logs(id) ON DELETE SET NULL,
  occurred_at TEXT NOT NULL,
  movement_type TEXT NOT NULL,
  quantity_delta REAL NOT NULL,
  quantity_after REAL NOT NULL,
  reason TEXT,
  notes TEXT,
  created_at TEXT NOT NULL,
  CHECK (quantity_delta <> 0),
  CHECK (quantity_after >= 0),
  CHECK (movement_type IN ('dose','manual_add','manual_remove','correction','discard','open_package','reconcile'))
);
CREATE INDEX idx_stock_movements_inventory_lot_id ON stock_movements(inventory_lot_id);
CREATE INDEX idx_stock_movements_product_id ON stock_movements(product_id);
CREATE INDEX idx_stock_movements_dose_log_id ON stock_movements(dose_log_id);
CREATE INDEX idx_stock_movements_occurred_at ON stock_movements(occurred_at);

CREATE TABLE dose_log_nutrients (
  id TEXT PRIMARY KEY,
  dose_log_id TEXT NOT NULL REFERENCES dose_logs(id) ON DELETE CASCADE,
  nutrient_id TEXT NOT NULL REFERENCES nutrients(id) ON DELETE RESTRICT,
  amount REAL NOT NULL,
  unit TEXT NOT NULL,
  reference_daily_amount REAL,
  reference_daily_unit TEXT,
  reference_source TEXT,
  percent_of_reference REAL,
  created_at TEXT NOT NULL,
  UNIQUE (dose_log_id, nutrient_id),
  CHECK (amount > 0),
  CHECK (length(trim(unit)) > 0),
  CHECK (reference_daily_amount IS NULL OR reference_daily_amount > 0)
);
CREATE INDEX idx_dose_log_nutrients_dose_log_id ON dose_log_nutrients(dose_log_id);
CREATE INDEX idx_dose_log_nutrients_nutrient_id ON dose_log_nutrients(nutrient_id);
