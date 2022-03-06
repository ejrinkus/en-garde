use crate::schema::characters::dsl::*;
use crate::schema::players::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::vec::Vec;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_players_with_characters() -> Vec<(i32, String, Option<i32>, Option<String>)> {
    // TODO - Don't panic if there's an error.
    let connection = establish_connection();
    // players
    //     .load::<Player>(&connection)
    //     .expect("Error loading players")
    players
        .left_join(characters)
        .select((
            crate::schema::players::columns::id,
            crate::schema::players::columns::name,
            crate::schema::characters::columns::id.nullable(),
            crate::schema::characters::columns::name.nullable(),
        ))
        .load(&connection)
        .expect("Error loading players")
}

pub fn insert_player(pname: String) {
    // TODO - Don't panic if there's an error.
    let connection = establish_connection();
    diesel::insert_into(players)
        .values(&crate::schema::players::columns::name.eq(&pname))
        .execute(&connection)
        .expect("Error inserting player");
}
