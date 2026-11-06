ALTER TABLE dose_log_nutrients ADD COLUMN nutrient_canonical_name TEXT;
ALTER TABLE dose_log_nutrients ADD COLUMN nutrient_display_name TEXT;
ALTER TABLE dose_log_nutrients ADD COLUMN nutrient_kind TEXT;

UPDATE dose_log_nutrients
SET
  nutrient_canonical_name = (SELECT canonical_name FROM nutrients WHERE nutrients.id = dose_log_nutrients.nutrient_id),
  nutrient_display_name = (SELECT display_name FROM nutrients WHERE nutrients.id = dose_log_nutrients.nutrient_id),
  nutrient_kind = (SELECT kind FROM nutrients WHERE nutrients.id = dose_log_nutrients.nutrient_id)
WHERE nutrient_display_name IS NULL;
