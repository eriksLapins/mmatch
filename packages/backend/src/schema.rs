// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, std::fmt::Debug, serde::Deserialize, serde::Serialize, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_types"))]
    pub struct UserTypes;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserTypes;

    users (id) {
        id -> Text,
        #[max_length = 125]
        name -> Varchar,
        #[max_length = 125]
        lastname -> Varchar,
        description -> Text,
        email -> Text,
        password -> Text,
        #[max_length = 125]
        phone -> Varchar,
        #[max_length = 125]
        phone_prefix -> Varchar,
        #[max_length = 125]
        country -> Varchar,
        #[max_length = 255]
        city -> Varchar,
        #[max_length = 255]
        street -> Varchar,
        #[max_length = 16]
        house_number -> Varchar,
        #[max_length = 16]
        apartment -> Nullable<Varchar>,
        types -> Array<Nullable<UserTypes>>,
    }
}
