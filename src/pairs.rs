use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::pairs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Pair {
    pub id: i32,
    pub spanish: String,
    pub english: String,
}

impl Pair {
    pub fn to_string(&self) -> String {
        let pair_str = format!("({}, {})", self.spanish, self.english);
        pair_str
    }

}