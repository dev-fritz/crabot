use chrono::NaiveDateTime;

struct TransactionModel {
    id: i32,
    amount: f64,
    description: Option<String>,
    user_id : i32,
    pix: Option<i32>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
    removed: Option<bool>,
    order: i32,
}