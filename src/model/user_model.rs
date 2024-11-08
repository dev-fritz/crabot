use chrono::NaiveDateTime;

enum UserType {
    Admin = 1,
    User = 2,
    Guest = 3
}

struct UserModel {
    id : i32,
    name : String,
    email : String,
    password : String,
    user_type : UserType,
    pix : Option<String>,
    doc : Option<String>,
    created_at : Option<NaiveDateTime>,
    updated_at : Option<NaiveDateTime>,
    removed: Option<bool>,
}