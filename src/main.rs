#![feature(decl_macro)]
use rocket::*;

use en_garde_libs::db::*;
use en_garde_libs::manipulations::*;
use en_garde_libs::model::*;
use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use std::collections::HashMap;

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
    let context = PlayersContext { list: &results };

    Template::render("players", context)
}

#[post("/add_player", data = "<form>")]
fn add_player(form: Form<NewPlayerForm>) -> String {
    let from_form = form.into_inner();
    let user = NewUser {
        username: from_form.name,
        permissions_mask: 0,
        pwd: "".to_string(),
    };
    insert_user(user);

    "Player added successfully".to_string()
}

#[post("/add_character", data = "<form>")]
fn add_character(form: Form<NewPlayerCharacterForm>) -> String {
    let from_form = form.into_inner();
    // There are two rolls involved with figuring out whether a character is an
    // orphan; figure that out here.
    let mut merged_sib = from_form.sib_roll;
    if merged_sib == 1 && from_form.orphan_roll == 1 {
        merged_sib = 0;
    }
    // Similar to siblings, there are two rolls to figure out the father's position.
    let mut merged_father = from_form.father_roll;
    let mut maybe_father_title: Option<i32> = None;
    let mut maybe_title: Option<i32> = None;
    if from_form.birth_roll > 2 {
        // Gentleman positions are 7-12.
        merged_father += 6;
    }
    if from_form.birth_roll > 4 {
        // Nobleman positions are 13-18.
        merged_father += 6;
        // In this case, the father also has a title.
        maybe_father_title = Some(from_form.title_roll);
        if merged_sib == 0 {
            // Since the character is an orphan, they inherit their father's
            // title.
            maybe_title = maybe_father_title;
        }
    }
    // Endurance doesn't need to be entered into the form; it's just strength *
    // constitution.
    let strength = from_form.strength;
    let constitution = from_form.constitution;
    let endurance = strength * constitution;
    // We need to query the rollable tables to figure out what the starting SL,
    // funds, and allowance are.
    let (sl, funds, allowance) =
        get_starting_resources(merged_sib, merged_father, maybe_father_title);
    // Now, shove everything into a struct to insert into the DB.
    let character = NewCharacter {
        user_id: from_form.player_id,
        name: from_form.name,
        abbr: from_form.abbr,
        birth_id: from_form.birth_roll,
        sib_id: merged_sib,
        father_id: merged_father,
        father_title_id: maybe_father_title,
        sl: sl,
        crowns: funds,
        allowance: allowance,
        strength: strength,
        expertise: from_form.expertise,
        constitution: constitution,
        endurance: endurance,
        military_ability: from_form.military_ability,
        club_id: None,
        mistress_id: None,
        rank_id: None,
        brevet_rank_id: None,
        title_id: maybe_title,
        turn_enlisted: None,
    };
    insert_character(character);

    "Character added successfully".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![index, players, add_player, add_character])
        .mount("/js", FileServer::from(relative!("/templates/js")))
        .mount("/styles", FileServer::from(relative!("/templates/styles")))
        .mount("/templates", FileServer::from(relative!("/templates")))
        .attach(Template::fairing())
}
