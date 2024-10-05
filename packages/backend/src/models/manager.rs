use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Manager {
    user: User,
    stage_name: String,
    commission: [f32; 2],
    bands: Vec<YearFromTo<Band>>,
    categories_interested_in: Vec<String>,
}

impl Manager {
    pub fn new(
        user: User,
        stage_name: String,
        commission: [f32; 2],
        bands: Vec<YearFromTo<Band>>,
        categories_interested_in: Vec<String>
    ) -> Self {
        Self {
            user,
            stage_name,
            commission,
            bands,
            categories_interested_in
        }
    }

    pub fn default() -> Self {
        let user = User::default();
        Self {
            user,
            stage_name: "".to_string(),
            commission: [0.0, 0.0],
            bands: vec![],
            categories_interested_in: vec![],
        }
    }
}
