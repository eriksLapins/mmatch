use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Selectable, PartialEq)]
#[diesel(table_name=year_from_to)]
pub struct YearFromTo {
    id: String,
    pub year_from: String,
    pub year_to: String,
    pub item_type: UserTypes,
    pub item_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to="YearFromTo.ts", rename="YearFromTo")]
pub struct GenYearFromTo<T: TS> {
    id: String,
    pub year_from: String,
    pub year_to: String,
    pub item: T
}

#[derive(Clone, Debug, Serialize, Deserialize, TS, Queryable, Insertable, AsChangeset, Selectable, PartialEq)]
#[diesel(table_name=skills)]
pub struct Skills {
    id: String,
    pub user_id: String,
    pub year_from: String,
    pub year_to: String,
    pub level: i16,
    pub name: String,
}