//! Helpers shared by dose-log write operations.

mod nutrient_snapshot_writer;
mod serving;
mod stock;

pub(super) use nutrient_snapshot_writer::{
    insert_dose_log_nutrient_snapshots, rescale_dose_log_nutrient_snapshots,
};
pub(super) use serving::validated_multiplier_for_product;
pub(super) use stock::{
    get_lot_for_update, normalize_stock_quantity, update_lot_stock, validate_lot_for_dose,
};
