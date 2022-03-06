#![feature(decl_macro)]
use rocket::*;

use en_garde_libs::db::*;
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
    let results = get_players_with_characters();

    // Wrap the query results in a serializable context to pass to the template.
    #[derive(Serialize)]
    struct PlayerContext {
        player_id: i32,
        player_name: String,
        character_id: Option<i32>,
        character_name: Option<String>,
    }
    #[derive(Serialize)]
    struct PlayersContext<'a> {
        list: &'a Vec<PlayerContext>,
    }
    let context_list = results
        .into_iter()
        .map(|(pid, pname, cid, cname)| PlayerContext {
            player_id: pid,
            player_name: pname,
            character_id: cid,
            character_name: cname,
        })
        .collect();
    let context = PlayersContext {
        list: &context_list,
    };

    Template::render("players", context)
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
