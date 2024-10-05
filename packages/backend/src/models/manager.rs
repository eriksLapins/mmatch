use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, TS, Insertable, Queryable, AsChangeset, Selectable, PartialEq)]
#[diesel(table_name=managers)]
#[ts(export)]
pub struct Manager {
    user_id: String,
    stage_name: String,
    commission: Vec<f32>,
    bands: Vec<YearFromTo<Band>>,
    categories_interested_in: Vec<String>,
}

impl diesel::Expression for Manager {
    type SqlType = diesel::sql_types::Json;
}

impl Manager {
    pub fn new(
        user_id: String,
        stage_name: String,
        commission: Vec<f32>,
        bands: Vec<YearFromTo<Band>>,
        categories_interested_in: Vec<String>
    ) -> Self {
        Self {
            user_id,
            stage_name,
            commission,
            bands,
            categories_interested_in
        }
    }

    pub fn default() -> Self {
        let user_id = User::default().id;
        Self {
            user_id,
            stage_name: "".to_string(),
            commission: vec![0.0, 0.0],
            bands: vec![],
            categories_interested_in: vec![],
        }
    }
}
