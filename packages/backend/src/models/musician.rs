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
    bands: Vec<GenYearFromTo<Band>>,
    managers: Option<Vec<GenYearFromTo<Manager>>>,
    links: Vec<String>,
    skills: Vec<String>,
    open_to_collab_with: Vec<String>,
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

    
    pub async fn get(Path(musician): Path<String>, state: Arc<AppState>) -> Result<(StatusCode, Json<MusicianResponse>), (StatusCode, Response<Body>)> {
        use crate::schema::musicians::dsl::*;
        let mut connection = establish_connection();

        let musician_response = musicians
            .select(Musician::as_select())
            .filter(id.eq(musician))
            .load::<Musician>(&mut connection);
        let musician_extracted = match musician_response {
                Ok(musician) => musician[0].clone(),
                Err(e) => return Err(errors::Error::new(StatusCode::NOT_FOUND, e.to_string(), None))
        };
        let user_response = User::get(Path(musician_extracted.user_id), state).await;

        let user = match user_response {
            Err(_e) => {
                let mut errors = serde_json::Map::new();
                errors.insert("user".to_string(), Value::String(format!("User not found")));
                return Err(errors::Error::new(StatusCode::INTERNAL_SERVER_ERROR, "User not found".to_string(), Some(errors.into())))
            }
            Ok((_status, Json(user))) => user
        };

        let filtered_open_to_collab_with: Vec<String> = musician_extracted.open_to_collab_with.into_iter().filter_map(|i| i ).collect();
        let filtered_skills: Vec<String> = musician_extracted.skills.into_iter().filter_map(|i| i ).collect();
        let filtered_links: Vec<String> = musician_extracted.links.into_iter().filter_map(|i| i ).collect();

        let response_musician = MusicianResponse {
            id: musician_extracted.id,
            stage_name: musician_extracted.stage_name,
            user,
            open_to_collab_with: filtered_open_to_collab_with,
            // TODO: when band has a get request
            bands: vec![],
            links: filtered_links,
            // TODO: when managers have a get request
            managers: None,
            skills: filtered_skills,
        };

        Ok((StatusCode::OK, Json(response_musician)))
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