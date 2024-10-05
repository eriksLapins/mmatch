// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, std::fmt::Debug, serde::Deserialize, serde::Serialize, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_types"))]
    pub struct UserTypes;
}

diesel::table! {
    bands (id) {
        id -> Text,
        #[max_length = 255]
        name -> Varchar,
        established_in -> Int4,
        description -> Text,
        #[max_length = 255]
        country_of_origin -> Varchar,
        members -> Array<Nullable<Json>>,
        music_styles -> Array<Nullable<Text>>,
        instruments -> Array<Nullable<Text>>,
        links -> Array<Nullable<Text>>,
        managers -> Nullable<Array<Nullable<Json>>>,
        searching_for -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    managers (id) {
        id -> Text,
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 255]
        stage_name -> Varchar,
        commission -> Array<Nullable<Float8>>,
        bands -> Array<Nullable<Json>>,
        categories_interested_in -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    musicians (id) {
        id -> Text,
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 255]
        stage_name -> Varchar,
        bands -> Array<Nullable<Json>>,
        managers -> Nullable<Array<Nullable<Json>>>,
        links -> Array<Nullable<Text>>,
        skills -> Array<Nullable<Text>>,
        open_to_collab_with -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    skills (id) {
        id -> Text,
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 125]
        year_from -> Varchar,
        #[max_length = 125]
        year_to -> Varchar,
        #[max_length = 125]
        name -> Varchar,
        level -> Int2,
    }
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

diesel::table! {
    year_from_to (id) {
        id -> Text,
        #[max_length = 255]
        user_band_manager -> Varchar,
        #[max_length = 125]
        year_from -> Varchar,
        #[max_length = 125]
        year_to -> Varchar,
        item -> Json,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bands,
    managers,
    musicians,
    skills,
    users,
    year_from_to,
);
