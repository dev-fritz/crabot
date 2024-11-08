use chrono::NaiveDateTime;

enum OrderStatus {
    Pending = 1,
    Processing = 2,
    Completed = 3,
    Cancelled = 4,
}

enum PaymentStatus {
    Pending = 1,
    Processing = 2,
    Completed = 3,
    Cancelled = 4,
}

enum PaymentMethod {
    Cash = 1,
    Card = 2,
    BankTransfer = 3,
    Pix = 4,
}

enum ResumeType {
    Conventional = 1,
    Simplified = 2,
    Complete = 3,
    Consolidated = 4,
}

struct OrderModel {
    id: i32,
    user_id: i32,
    order_status: OrderStatus,
    payment_status: PaymentStatus,
    payment_method: Option<PaymentMethod>,
    resume_type: ResumeType,
    quantity: i32,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}