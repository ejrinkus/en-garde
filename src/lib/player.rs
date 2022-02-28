use crate::db::*;
use crate::model::Player;
use crate::schema::players::dsl::*;
use diesel::prelude::*;
use rand::Rng;
use rocket::form::Form;
use rocket::serde::Serialize;
use rocket::*;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::sync::Mutex;
use std::vec::Vec;

lazy_static! {
    static ref PLAYERS: Mutex<HashMap<i32, Player>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

#[get("/players")]
pub fn get_players() -> Template {
    // Connect to the db and query for all players.
    let connection = establish_connection();
    let results = players
        .load::<Player>(&connection)
        .expect("Error loading players");

    // Create a wrapper context to hold a list of players.
    #[derive(Serialize)]
    struct PlayersContext<'a> {
        player_list: &'a Vec<Player>,
    }
    let list = PlayersContext {
        player_list: &results,
    };

    // DEBUGGING.
    list.player_list.iter().for_each(|p| println!("{:?}", p));

    // Render the first player to the template.
    Template::render("players", list)
}

#[get("/player?<pid>")]
pub fn player(pid: i32) -> Template {
    // Connect to the db and query for the player with the given ID.
    let connection = establish_connection();
    let result = players.filter(id.eq(pid)).first::<Player>(&connection);

    // If the player was found, render that player to the template.  Otherwise,
    // render an empty object (the template will display a "not found" message
    // to the user in this case).
    match result {
        Ok(p) => Template::render("player", p),
        Err(_) => Template::render("player", {}),
    }
}

fn gen_id(db: &HashMap<i32, Player>) -> i32 {
    let mut rng = rand::thread_rng();

    let mut number: i32 = rng.gen();
    while db.contains_key(&number) {
        number = rng.gen::<i32>();
    }
    number
}

#[derive(FromForm)]
pub struct PlayerName {
    player_name: String,
}

#[post("/add_player", data = "<name_form>")]
pub fn add_player(name_form: Form<PlayerName>) -> String {
    // Insert the player into the db.
    let pname = name_form.into_inner().player_name;
    let connection = establish_connection();
    diesel::insert_into(players)
        .values(&name.eq(&pname))
        .execute(&connection);

    // Return a success message.
    format!("Player added successfully: {:?}", pname)
}
