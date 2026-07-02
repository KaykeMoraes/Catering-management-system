pub enum CustomerType {
    Person(String),
    Company(String),
}

pub enum QuoteStatus {
    Draft,
    Sent,
    Accepted,
    Rejected,
    Expired,
}

pub enum PaymentMethod {
    Cash(String),
    Card(String),
    Check(String),
    Pix(String),
    BankTransfer(String),
}

pub enum PaymentStatus {
    Pending(String),
    Paid(String),
    Refunded(String),
    Cancelled(String),
}
