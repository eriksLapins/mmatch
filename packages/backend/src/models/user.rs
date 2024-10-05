use diesel::Selectable;
use std::fmt::Display;
use crate::prelude::*;


#[derive(Clone, Copy, Debug, Serialize, Deserialize, TS, diesel_derive_enum::DbEnum, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::UserTypes"]
pub enum UserTypes {
    Musician,
    Manager,
    Explorer
}

impl Display for UserTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, TS, Insertable, Queryable, AsChangeset, Selectable, PartialEq)]
#[diesel(table_name = users)]
#[ts(export)]
pub struct User {
    pub id: String,
    pub name: String,
    pub lastname: String,
    pub description: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub phone_prefix: String,
    pub country: String,
    pub city: String,
    pub street: String,
    pub house_number: String,
    pub apartment: Option<String>,
    pub types: Vec<Option<UserTypes>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CreateUserPayload {
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
    pub types: Vec<Option<UserTypes>>,
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
        types: Vec<Option<UserTypes>>,
    ) -> Self {
        let user_id = uuid::Uuid::new_v4().to_string();

        Self {
            id: user_id,
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
    pub async fn create(Json(payload): Json<CreateUserPayload>, _state: Arc<AppState>) -> Result<StatusCode, (StatusCode, Response<Body>)> {
        use crate::schema::users::dsl::*;
        let mut connection = establish_connection();
        let user = User::new(
            payload.name,
            payload.lastname,
            payload.description,
            payload.email,
            payload.phone,
            payload.phone_prefix,
            payload.country,
            payload.city,
            payload.street,
            payload.house_number,
            payload.apartment,
            payload.password,
            payload.types,
        );

        
        let mut errors = serde_json::Map::new();
        if &user.name == "" {
            errors.insert("name".to_string(), Value::String("Name is required".to_string()));
        }
        if &user.lastname == "" {
            errors.insert("lastname".to_string(), Value::String("Lastname is required".to_string()));
        }
        if &user.description == "" {
            errors.insert("description".to_string(), Value::String("description is required".to_string()));
        }
        if &user.phone == "" {
            errors.insert("phone".to_string(), Value::String("phone is required".to_string()));
        }
        if &user.phone_prefix == "" {
            errors.insert("phone_prefix".to_string(), Value::String("phone_prefix is required".to_string()));
        }
        if &user.country == "" {
            errors.insert("country".to_string(), Value::String("country is required".to_string()));
        }
        if &user.city == "" {
            errors.insert("city".to_string(), Value::String("city is required".to_string()));
        }
        if &user.street == "" {
            errors.insert("street".to_string(), Value::String("street is required".to_string()));
        }
        if &user.house_number == "" {
            errors.insert("house_number".to_string(), Value::String("house_number is required".to_string()));
        }
        if &user.password == "" {
            errors.insert("password".to_string(), Value::String("password is required".to_string()));
        }

        if errors.is_empty() == false {
            return Err(errors::Error::new(StatusCode::UNPROCESSABLE_ENTITY, StatusCode::UNPROCESSABLE_ENTITY.to_string(), Some(errors.into())))
        }

        let user_result = diesel::insert_into(users)
            .values(&user)
            .execute(&mut connection);

        match user_result {
            Err(e) => Err(errors::Error::new(StatusCode::BAD_REQUEST, e.to_string(), None)),
            Ok(_) => Ok(StatusCode::OK)
        }
    }
    pub async fn get(Path(user): Path<String>, _state: Arc<AppState>) -> Result<Json<Vec<User>>, (StatusCode, Response<Body>)> {
        use crate::schema::users::dsl::*;
        let mut connection = establish_connection();

        let user = users
            .select(User::as_select())
            .filter(id.eq(user))
            .load::<User>(&mut connection);
            
        match user {
            Ok(user) => Ok(Json(user)),
            Err(e) => Err(errors::Error::new(StatusCode::NOT_FOUND, e.to_string(), None))
        }
    }
    
    pub async fn get_all(_state: Arc<AppState>) -> Json<Vec<User>> {
        use crate::schema::users::dsl::*;
        let mut connection = establish_connection();

        let user = users
            .select(User::as_select())
            .load::<User>(&mut connection)
            .expect("Failed to load users");
        Json(user)
    }
}