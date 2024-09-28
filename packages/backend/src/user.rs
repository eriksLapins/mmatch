use std::fmt::Display;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
pub struct YearFromTo<T> {
    pub from: String,
    pub to: String,
    pub item: T,
}


#[derive(Clone, Copy, Debug, Serialize, Deserialize, TS)]
pub enum UserTypes {
    Musician,
    Manager,
    Explorer,
}

impl Display for UserTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct User {
    pub id: String,
    pub name: String,
    pub lastname: String,
    pub description: String,
    pub email: String,
    pub phone: String,
    pub phone_prefix: String,
    pub country: String,
    pub city: String,
    pub street: String,
    pub house_number: String,
    pub apartment: Option<String>,
    pub password: String,
    pub types: Vec<UserTypes>,
}

impl User {
    pub fn new(
        name: String,
        lastname: String,
        description: String,
        email: String,
        phone: String,
        phone_prefix: String,
        country: String,
        city: String,
        street: String,
        house_number: String,
        apartment: Option<String>,
        password: String,
        types: Vec<UserTypes>,
    ) -> Self {
        let id = uuid::Uuid::new_v4();
        Self {
            id: id.to_string(),
            name,
            lastname,
            description,
            email,
            phone,
            phone_prefix,
            country,
            city,
            street,
            house_number,
            apartment,
            password,
            types,    
        }
    }

    pub fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "".to_string(),
            lastname: "".to_string(),
            description: "".to_string(),
            email: "".to_string(),
            phone: "".to_string(),
            phone_prefix: "".to_string(),
            country: "".to_string(),
            city: "".to_string(),
            street: "".to_string(),
            house_number: "".to_string(),
            apartment: None,
            password: "".to_string(),
            types: vec![], 
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
pub struct Skills {
    from: String,
    to: String,
    level: i8,
    name: String,
}

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

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
pub struct MusicianWithPurpose {
    musician: Musician,
    main_purpose: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Band {
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