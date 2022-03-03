use crate::db::*;
use crate::model::Player;
use crate::schema::players::dsl::*;
use diesel::prelude::*;
use std::vec::Vec;

pub fn get_players() -> Vec<Player> {
    // TODO - Don't panic if there's an error.
    let connection = establish_connection();
    players
        .load::<Player>(&connection)
        .expect("Error loading players")
}

pub fn insert_player(pname: String) {
    // TODO - Don't panic if there's an error.
    let connection = establish_connection();
    diesel::insert_into(players)
        .values(&name.eq(&pname))
        .execute(&connection)
        .expect("Error inserting player");
}
