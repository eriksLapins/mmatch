use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, TS, Insertable, Queryable, AsChangeset, Selectable, PartialEq)]
#[diesel(table_name = musicians)]
#[ts(export)]
pub struct Musician {
    id: String,
    user_id: String,
    stage_name: String,
    bands: Vec<YearFromTo<Band>>,
    skills: Vec<Skills>,
    links: Vec<String>,
    managers: Option<Vec<YearFromTo<Manager>>>,
    open_to_collab_with: Vec<String>,
}
impl diesel::Expression for Musician {
    type SqlType = diesel::sql_types::Json;
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CreateMusicianPayload {
    user_id: String,
    stage_name: String,
    bands: Vec<YearFromTo<Band>>,
    skills: Vec<Skills>,
    links: Vec<String>,
    managers: Option<Vec<YearFromTo<Manager>>>,
    open_to_collab_with: Vec<String>,
}

impl Musician {
    pub fn new(
        user_id: String,
        stage_name: String,
        bands: Vec<YearFromTo<Band>>,
        skills: Vec<Skills>,
        links: Vec<String>,
        managers: Option<Vec<YearFromTo<Manager>>>,
        open_to_collab_with: Vec<String>,
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
        let Json(user) = User::get(Path(payload.user_id), state).await.unwrap();
        if user.len() == 0 {
            return Err(errors::Error::new(StatusCode::NOT_FOUND, "User not found".to_string(), Some(user_id_clone.into())))
        }
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

        
        let mut errors = serde_json::Map::new();
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
            Ok(_) => Ok(StatusCode::OK)
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
pub struct MusicianWithPurpose {
    musician: Musician,
    main_purpose: String,
}

impl diesel::Expression for MusicianWithPurpose {
    type SqlType = diesel::sql_types::Json;
}