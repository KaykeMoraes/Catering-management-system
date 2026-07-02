use chrono::{DateTime, Utc};

pub struct Event {
    id: u64,
    date_time: DateTime<Utc>,
    guest_count: i32,
    customer_id: u64,
    event_type: String,
    payment_status: String,
    notes: Option<String>,
}
