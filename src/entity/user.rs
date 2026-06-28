use chrono::{DateTime, Utc};

pub struct User {
    id: u64,
    email: String,
    password: String,
    username: String,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
