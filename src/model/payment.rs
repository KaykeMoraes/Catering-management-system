use chrono::{DateTime, Utc};

use crate::utils::{PaymentMethod, PaymentStatus};

pub struct Payment {
    id: u64,
    event_id: u64,
    amount: f64,
    payment_method: PaymentMethod,
    payment_status: PaymentStatus,
    created_at: DateTime<Utc>,
    paid_at: Option<DateTime<Utc>>,
}
