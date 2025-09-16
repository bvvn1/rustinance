use uuid::Uuid;

pub struct Account {
    id: Uuid,
    name: String,
    balance: u64,
    password_hash: String,
}
