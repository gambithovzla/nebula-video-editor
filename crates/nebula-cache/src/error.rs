use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("cache budget exceeded")]
    BudgetExceeded,
}
