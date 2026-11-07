//! Scenariusze testowe dawkowania i ruchów magazynowych w SQLite

#[path = "inventory_dosing/cancel.rs"]
mod cancel;
#[path = "inventory_dosing/create.rs"]
mod create;
#[path = "inventory_dosing/fixture.rs"]
mod fixture;
#[path = "inventory_dosing/history.rs"]
mod history;
#[path = "inventory_dosing/lot_identity.rs"]
mod lot_identity;
#[path = "inventory_dosing/lot_status.rs"]
mod lot_status;
#[path = "inventory_dosing/serving.rs"]
mod serving;
#[path = "inventory_dosing/update.rs"]
mod update;
