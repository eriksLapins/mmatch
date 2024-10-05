use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, TS, Insertable, Queryable, AsChangeset, Selectable, PartialEq)]
#[diesel(table_name=bands)]
#[ts(export)]
pub struct Band {
    name: String,
    established_in: i32,
    description: String,
    country_of_origin: String,
    members: Vec<YearFromTo<MusicianWithPurpose>>,
    music_styles: Vec<String>,
    instruments: Vec<String>,
    links: Vec<String>,
    managers: Option<Vec<YearFromTo<Manager>>>,
    searching_for: Vec<String>,
}

impl diesel::Expression for Band {
    type SqlType = diesel::sql_types::Json;
}

impl Band {
    pub fn new(
        name: String,
        established_in: i16,
        description: String,
        country_of_origin: String,
        members: Vec<YearFromTo<MusicianWithPurpose>>,
        music_styles: Vec<String>,
        instruments: Vec<String>,
        links: Vec<String>,
        managers: Option<Vec<YearFromTo<Manager>>>,
        searching_for: Vec<String>,
    ) -> Self {
        Self {
            name,
            established_in,
            description,
            country_of_origin,
            members,
            music_styles,
            instruments,
            links,
            managers,
            searching_for,
        }
    }

    pub fn default() -> Self {
        Self {
            name: "".to_string(),
            established_in: 1990,
            description: "".to_string(),
            country_of_origin: "".to_string(),
            members: vec![],
            music_styles: vec![],
            instruments: vec![],
            links: vec![],
            managers: None,
            searching_for: vec![],
        }
    }
}