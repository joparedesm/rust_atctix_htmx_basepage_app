use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pair {
    pub spanish: String,
    pub english: String,
}

impl Pair {
    pub fn to_string(&self) -> String {
        let pair_str = format!("({}, {})", self.spanish, self.english);
        pair_str
    }

}