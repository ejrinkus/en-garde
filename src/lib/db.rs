use crate::model::*;
use crate::schema::characters::dsl::*;
use crate::schema::father_positions::dsl::*;
use crate::schema::sibling_ranks::dsl::*;
use crate::schema::titles::dsl::*;
use crate::schema::users::dsl::*;
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

pub fn get_players_with_characters() -> Vec<PlayerAndCharacter> {
    // TODO - Don't panic if there's an error.
    let connection = establish_connection();
    users
        .left_join(characters)
        .select((
            crate::schema::users::columns::id,
            crate::schema::users::columns::username,
            crate::schema::characters::columns::id.nullable(),
            crate::schema::characters::columns::name.nullable(),
        ))
        .load::<PlayerAndCharacter>(&connection)
        .expect("Error loading players")
}

pub fn insert_user(new_user: NewUser) {
    // TODO - Don't panic if there's an error.
    let connection = establish_connection();
    diesel::insert_into(users)
        .values(new_user)
        .execute(&connection)
        .expect("Error inserting user");
}

pub fn get_sib_rank(rid: i32) -> SiblingRank {
    // TODO - Don't panic if there's an error.
    let connection = establish_connection();
    sibling_ranks
        .filter(crate::schema::sibling_ranks::columns::id.eq(rid))
        .first::<SiblingRank>(&connection)
        .expect("Error loading sibling rank")
}

pub fn get_father_pos(fpid: i32) -> FatherPosition {
    // TODO - Don't panic if there's an error.
    let connection = establish_connection();
    father_positions
        .filter(crate::schema::father_positions::columns::id.eq(fpid))
        .first::<FatherPosition>(&connection)
        .expect("Error loading father position")
}

pub fn get_title(tid: i32) -> Title {
    // TODO - Don't panic if there's an error.
    let connection = establish_connection();
    titles
        .filter(crate::schema::titles::columns::id.eq(tid))
        .first::<Title>(&connection)
        .expect("Error loading titles")
}

pub fn insert_character(new_character: NewCharacter) {
    // TODO - Don't panic if there's an error.
    let connection = establish_connection();
    diesel::insert_into(characters)
        .values(new_character)
        .execute(&connection)
        .expect("Error inserting character");
}
