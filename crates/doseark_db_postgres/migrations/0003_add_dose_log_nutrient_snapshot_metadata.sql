ALTER TABLE dose_log_nutrients
ADD COLUMN IF NOT EXISTS nutrient_canonical_name TEXT,
ADD COLUMN IF NOT EXISTS nutrient_display_name TEXT,
ADD COLUMN IF NOT EXISTS nutrient_kind TEXT;

UPDATE dose_log_nutrients dln
SET
  nutrient_canonical_name = n.canonical_name,
  nutrient_display_name = n.display_name,
  nutrient_kind = n.kind
FROM nutrients n
WHERE dln.nutrient_id = n.id
  AND dln.nutrient_display_name IS NULL;
