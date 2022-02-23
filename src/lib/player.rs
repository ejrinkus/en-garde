use crate::model::Player;
use rand::Rng;
use rocket::form::{Form, Lenient};
use rocket::*;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref PLAYERS: Mutex<HashMap<u64, Player>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

#[get("/player?<id>")]
pub fn player(id: u64) -> Template {
    // Get the lock on the in-mem players db.
    let players = PLAYERS.lock().unwrap();

    // Default player for "ID not found" situations.
    let mut player = Player {
        id: id,
        name: String::from("NOT_FOUND"),
    };

    // Try to find the player.
    let maybe_player = players.get(&id);
    if maybe_player.is_some() {
        player = maybe_player.unwrap().clone();
    }

    // Render the player to the template.
    Template::render("player", player)
}

#[get("/new_player")]
pub fn new_player() -> Template {
    Template::render("new_player", ())
}

fn gen_id(db: &HashMap<u64, Player>) -> u64 {
    let mut rng = rand::thread_rng();

    let mut number: u64 = rng.gen();
    while db.contains_key(&number) {
        number = rng.gen::<u64>();
    }
    number
}

#[post("/add_player", data = "<player_form>")]
pub fn add_player(player_form: Form<Lenient<Player>>) -> String {
    // Get the lock on the in-mem players db.
    let mut players = PLAYERS.lock().unwrap();

    // Convert the form to a player object and add it to the db.
    let mut player: Player = player_form.into_inner().into_inner();
    player.id = gen_id(&players);
    players.insert(player.id, player);

    // Return a success message.
    format!("Player added successfully: {:?}", players)
}
