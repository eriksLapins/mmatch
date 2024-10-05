use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, Queryable, Insertable, PartialEq)]
#[diesel(table_name=year_from_to)]
pub struct YearFromTo<T: diesel::Expression<SqlType = diesel::sql_types::Json> + TS> {
    id: String,
    pub user_band_manager: String,
    pub year_from: String,
    pub year_to: String,
    pub item: T,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS, Queryable, Insertable, PartialEq)]
#[diesel(table_name=skills)]
pub struct Skills {
    id: String,
    pub user_id: String,
    pub year_from: String,
    pub year_to: String,
    pub level: i16,
    pub name: String,
}