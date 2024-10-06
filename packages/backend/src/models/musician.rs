use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, TS, Insertable, Queryable, AsChangeset, Selectable, PartialEq)]
#[diesel(table_name = musicians)]
pub struct Musician {
    id: String,
    user_id: String,
    stage_name: String,
    // year_from_to band
    bands: Vec<Option<String>>,
    // year from to manager
    managers: Option<Vec<Option<String>>>,
    links: Vec<Option<String>>,
    skills: Vec<Option<String>>,
    open_to_collab_with: Vec<Option<String>>,
}
impl diesel::Expression for Musician {
    type SqlType = diesel::sql_types::Json;
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export, rename="Musician", export_to="Musician.ts")]
pub struct MusicianResponse {
    id: String,
    user: User,
    stage_name: String,
    bands: Vec<Option<GenYearFromTo<Band>>>,
    managers: Option<Vec<GenYearFromTo<Manager>>>,
    links: Vec<Option<String>>,
    skills: Vec<Option<String>>,
    open_to_collab_with: Vec<Option<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CreateMusicianPayload {
    user_id: String,
    stage_name: String,
    bands: Vec<Option<String>>,
    skills: Vec<Option<String>>,
    links: Vec<Option<String>>,
    managers: Option<Vec<Option<String>>>,
    open_to_collab_with: Vec<Option<String>>,
}

impl Musician {
    pub fn new(
        user_id: String,
        stage_name: String,
        bands: Vec<Option<String>>,
        skills: Vec<Option<String>>,
        links: Vec<Option<String>>,
        managers: Option<Vec<Option<String>>>,
        open_to_collab_with: Vec<Option<String>>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            stage_name,
            bands,
            skills,
            links,
            managers,
            open_to_collab_with
        }
    }

    pub fn default() -> Self {
        let user_id = User::default().id;
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            stage_name: "".to_string(),
            bands: vec![],
            skills: vec![],
            links: vec![],
            managers: None,
            open_to_collab_with: vec![],
        }
    }

    pub async fn create(Json(payload): Json<CreateMusicianPayload>, state: Arc<AppState>) -> Result<StatusCode, (StatusCode, Response<Body>)> {
        use crate::schema::musicians::dsl::*;
        let user_id_clone = payload.user_id.clone();
        let user_response = User::get(Path(payload.user_id), state).await;
        let mut errors = serde_json::Map::new();
        match user_response {
            Err(_e) => {
                errors.insert("user_id".to_string(), Value::String(format!("User not found")));
                return Err(errors::Error::new(StatusCode::NOT_FOUND, "User not found".to_string(), Some(errors.into())))
            }
            Ok(_) => ()
        };
        let mut connection = establish_connection();
        let musician = Musician::new(
            user_id_clone,
            payload.stage_name,
            payload.bands,
            payload.skills,
            payload.links,
            payload.managers,
            payload.open_to_collab_with,
        );

        if &musician.stage_name == "" {
            errors.insert("stage_name".to_string(), Value::String("Stage name is required".to_string()));
        }

        if errors.is_empty() == false {
            return Err(errors::Error::new(StatusCode::UNPROCESSABLE_ENTITY, StatusCode::UNPROCESSABLE_ENTITY.to_string(), Some(errors.into())))
        }

        let musician_result = diesel::insert_into(musicians)
            .values(&musician)
            .execute(&mut connection);

        match musician_result {
            Err(e) => Err(errors::Error::new(StatusCode::BAD_REQUEST, e.to_string(), None)),
            Ok(_) => Ok(StatusCode::CREATED)
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Queryable, AsChangeset, Selectable, PartialEq)]
#[diesel(table_name=musician_with_purpose)]
pub struct DbMusicianWithPurpose {
    band_id: String,
    musician_id: String,
    main_purpose: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct MusicianWithPurpose {
    musician: MusicianResponse,
    main_purpose: String,
}

impl diesel::Expression for MusicianWithPurpose {
    type SqlType = diesel::sql_types::Json;
}