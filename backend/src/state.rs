use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::repositories::*;

#[cfg(test)]
use crate::repositories::{
    MockContactRepo, MockLedgerAccountRepo, MockLedgerTransactionRepo, MockUserRepo,
};

#[derive(Clone)]
pub struct AppState {
    pub contacts: Arc<dyn ContactRepo>,
    pub ledger_accounts: Arc<dyn LedgerAccountRepo>,
    pub ledger_transactions: Arc<dyn LedgerTransactionRepo>,
    pub products: Arc<dyn ProductRepo>,
    pub stocks: Arc<dyn StockRepo>,
    pub stock_moves: Arc<dyn StockMoveRepo>,
    pub taxes: Arc<dyn TaxRepo>,
    pub users: Arc<dyn UserRepo>,
    pub variants: Arc<dyn VariantRepo>,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            contacts: Arc::new(ContactRepoImpl::new(db.clone())),
            ledger_accounts: Arc::new(LedgerAccountRepoImpl::new(db.clone())),
            ledger_transactions: Arc::new(LedgerTransactionRepoImpl::new(db.clone())),
            products: Arc::new(ProductRepoImpl::new(db.clone())),
            stocks: Arc::new(StockRepoImpl::new(db.clone())),
            stock_moves: Arc::new(StockMoveRepoImpl::new(db.clone())),
            taxes: Arc::new(TaxRepoImpl::new(db.clone())),
            users: Arc::new(UserRepoImpl::new(db.clone())),
            variants: Arc::new(VariantRepoImpl::new(db.clone())),
        }
    }
}

#[cfg(test)]
pub struct MockAppState {
    pub contacts: MockContactRepo,
    pub ledger_accounts: MockLedgerAccountRepo,
    pub ledger_transactions: MockLedgerTransactionRepo,
    pub products: MockProductRepo,
    pub stocks: MockStockRepo,
    pub stock_moves: MockStockMoveRepo,
    pub taxes: MockTaxRepo,
    pub users: MockUserRepo,
    pub variants: MockVariantRepo,
}

#[cfg(test)]
impl From<MockAppState> for AppState {
    fn from(value: MockAppState) -> Self {
        AppState {
            contacts: Arc::new(value.contacts),
            ledger_accounts: Arc::new(value.ledger_accounts),
            ledger_transactions: Arc::new(value.ledger_transactions),
            products: Arc::new(value.products),
            stocks: Arc::new(value.stocks),
            stock_moves: Arc::new(value.stock_moves),
            taxes: Arc::new(value.taxes),
            users: Arc::new(value.users),
            variants: Arc::new(value.variants),
        }
    }
}

#[cfg(test)]
impl MockAppState {
    pub fn new() -> Self {
        Self {
            contacts: MockContactRepo::new(),
            ledger_accounts: MockLedgerAccountRepo::new(),
            ledger_transactions: MockLedgerTransactionRepo::new(),
            products: MockProductRepo::new(),
            stocks: MockStockRepo::new(),
            stock_moves: MockStockMoveRepo::new(),
            taxes: MockTaxRepo::new(),
            users: MockUserRepo::new(),
            variants: MockVariantRepo::new(),
        }
    }
}
