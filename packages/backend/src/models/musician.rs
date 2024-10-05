use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Musician {
    user: User,
    stage_name: String,
    bands: Vec<YearFromTo<Band>>,
    skills: Vec<Skills>,
    links: Vec<String>,
    managers: Option<Vec<YearFromTo<Manager>>>,
    open_to_collab_with: Vec<String>,
}

impl Musician {
    pub fn new(
        user: User,
        stage_name: String,
        bands: Vec<YearFromTo<Band>>,
        skills: Vec<Skills>,
        links: Vec<String>,
        managers: Option<Vec<YearFromTo<Manager>>>,
        open_to_collab_with: Vec<String>,
    ) -> Self {
        Self {
            user,
            stage_name,
            bands,
            skills,
            links,
            managers,
            open_to_collab_with
        }
    }

    pub fn default() -> Self {
        let user = User::default();
        Self {
            user,
            stage_name: "".to_string(),
            bands: vec![],
            skills: vec![],
            links: vec![],
            managers: None,
            open_to_collab_with: vec![],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
pub struct MusicianWithPurpose {
    musician: Musician,
    main_purpose: String,
}