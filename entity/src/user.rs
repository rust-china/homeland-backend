use arel::prelude::*;

#[arel_enum]
enum State {
    #[arel_enum(value = 0, default = true)]
    Diabled,
    #[arel_enum(value = 1)]
    Normal,
}

#[arel(table_name = "users")]
pub struct User {
    id: i32,
    username: String,
    email: String,
    state: State,
    created_at: chrono::DateTime<chrono::FixedOffset>,
    updated_at: chrono::DateTime<chrono::FixedOffset>,
    login_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}
impl Arel for User {}
