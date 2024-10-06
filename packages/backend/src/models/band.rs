use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, TS, Insertable, Queryable, AsChangeset, Selectable, PartialEq)]
#[diesel(table_name=bands)]
pub struct Band {
    name: String,
    established_in: i32,
    description: String,
    country_of_origin: String,
    members: Vec<Option<String>>,
    music_styles: Vec<String>,
    instruments: Vec<String>,
    links: Vec<String>,
    managers: Option<Vec<Option<String>>>,
    searching_for: Vec<String>,
}

impl diesel::Expression for Band {
    type SqlType = diesel::sql_types::Json;
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export, rename="Band", export_to="Band.ts")]
pub struct BandResponse {
    name: String,
    established_in: i32,
    description: String,
    country_of_origin: String,
    members: Vec<MusicianWithPurpose>,
    music_styles: Vec<String>,
    instruments: Vec<String>,
    links: Vec<String>,
    managers: Option<Vec<GenYearFromTo<Manager>>>,
    searching_for: Vec<String>,
}

impl Band {
    pub fn new(
        name: String,
        established_in: i32,
        description: String,
        country_of_origin: String,
        members: Vec<Option<String>>,
        music_styles: Vec<String>,
        instruments: Vec<String>,
        links: Vec<String>,
        managers: Option<Vec<Option<String>>>,
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