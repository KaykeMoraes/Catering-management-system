use chrono::{DateTime, Utc};

use crate::utils::CustomerType;

pub struct Customer {
    id: u64,
    user_id: u64,
    customer_type: CustomerType,
    full_name: String,
    company_name: Option<String>,
    cpf: Option<String>,
    cnpj: Option<String>,
    email: String,
    phone: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
