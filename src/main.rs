#![feature(decl_macro)]
use rocket::*;

use en_garde_libs::model::Context;
use en_garde_libs::player::*;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

#[get("/")]
fn index() -> Template {
    let context = Context {
        first_name: String::from("Eric"),
        last_name: String::from("Rinkus"),
    };
    Template::render("home", context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![index, player, get_players, add_player])
        .mount("/js", FileServer::from(relative!("/templates/js")))
        .mount("/styles", FileServer::from(relative!("/templates/styles")))
        .mount("/templates", FileServer::from(relative!("/templates")))
        .attach(Template::fairing())
}
