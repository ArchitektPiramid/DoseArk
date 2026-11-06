ALTER TABLE dose_logs ADD COLUMN status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active','cancelled'));
ALTER TABLE dose_logs ADD COLUMN cancelled_at TEXT;
ALTER TABLE dose_logs ADD COLUMN cancellation_reason TEXT;
CREATE INDEX idx_dose_logs_status ON dose_logs(status);
