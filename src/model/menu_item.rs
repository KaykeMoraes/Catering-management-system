use chrono::{DateTime, Utc};

pub struct MenuItem {
    id: u64,
    name: String,
    description: String,
    unit_price: f64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>
}
