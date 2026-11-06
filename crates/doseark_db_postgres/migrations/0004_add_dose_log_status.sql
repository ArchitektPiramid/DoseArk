ALTER TABLE dose_logs
ADD COLUMN IF NOT EXISTS status TEXT NOT NULL DEFAULT 'active',
ADD COLUMN IF NOT EXISTS cancelled_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS cancellation_reason TEXT;

ALTER TABLE dose_logs
DROP CONSTRAINT IF EXISTS chk_dose_logs_status;

ALTER TABLE dose_logs
ADD CONSTRAINT chk_dose_logs_status
CHECK (status IN ('active', 'cancelled'));

CREATE INDEX IF NOT EXISTS idx_dose_logs_status
ON dose_logs(status);
