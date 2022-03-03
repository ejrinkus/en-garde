#![feature(decl_macro)]
use rocket::*;

use en_garde_libs::model::*;
use en_garde_libs::player::*;
use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::vec::Vec;

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

#[get("/")]
fn index() -> Template {
    // Homepage doesn't need a template.
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("index", context)
}

#[get("/players")]
fn players() -> Template {
    let results = get_players();

    // Wrap the players list in a serializable context to pass to the template.
    #[derive(Serialize)]
    struct PlayersContext<'a> {
        player_list: &'a Vec<Player>,
    }
    let list = PlayersContext {
        player_list: &results,
    };

    Template::render("players", list)
}

#[derive(FromForm)]
struct AddPlayerForm {
    name: String,
}

#[post("/add_player", data = "<form>")]
fn add_player(form: Form<AddPlayerForm>) -> String {
    insert_player(form.into_inner().name);

    "Player added successfully".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![index, players, add_player])
        .mount("/js", FileServer::from(relative!("/templates/js")))
        .mount("/styles", FileServer::from(relative!("/templates/styles")))
        .mount("/templates", FileServer::from(relative!("/templates")))
        .attach(Template::fairing())
}
