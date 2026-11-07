//! Odczyt historii ruchów magazynowych

use crate::{ApplicationResult, StockMovementRepository};
use doseark_domain::StockMovement;
use std::sync::Arc;
use uuid::Uuid;

/// Reads recorded stock movements for inventory lots
#[derive(Clone)]
pub struct StockMovementService {
    repo: Arc<dyn StockMovementRepository>,
}

impl StockMovementService {
    pub fn new(repo: Arc<dyn StockMovementRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_stock_movements(&self) -> ApplicationResult<Vec<StockMovement>> {
        self.repo.list_stock_movements().await
    }

    pub async fn list_stock_movements_for_lot(
        &self,
        inventory_lot_id: Uuid,
    ) -> ApplicationResult<Vec<StockMovement>> {
        self.repo
            .list_stock_movements_for_lot(inventory_lot_id)
            .await
    }
}
