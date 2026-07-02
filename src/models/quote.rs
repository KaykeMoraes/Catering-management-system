use chrono::{DateTime, Utc};

use crate::utils::QuoteStatus;

pub struct Quote {
    pub id: u64,
    pub customer_id: u64,
    pub status: QuoteStatus,
    pub subtotal: f64,
    pub discount: f64,
    pub total: f64,
    pub notes: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
