use arel::prelude::*;

#[arel_enum]
pub enum State {
    #[arel_enum(value = 0, default = true)]
    Diabled,
    #[arel_enum(value = 1)]
    Normal,
}

#[arel(table_name = "users")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub state: State,
    pub created_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub updated_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub login_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}
impl Arel for User {}

impl User {
    pub fn is_valid(&self) -> bool {
        self.state == State::Normal
    }
}
