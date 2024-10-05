use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
pub struct YearFromTo<T> {
    pub from: String,
    pub to: String,
    pub item: T,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
pub struct Skills {
    from: String,
    to: String,
    level: i8,
    name: String,
}