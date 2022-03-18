use crate::schema::*;
use rocket::serde::Serialize;
use rocket::*;
use std::vec::Vec;

/* Structs for defining and handling users and user entry. */

#[derive(Clone, Debug, Identifiable, Queryable, Serialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub permissions_mask: i32,
    pub pwd: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub permissions_mask: i32,
    pub pwd: String,
}

#[derive(FromForm)]
pub struct NewPlayerForm {
    pub name: String,
}

/* Structs for pairing users with the character(s) they control. */

#[derive(Serialize)]
pub struct PlayersContext<'a> {
    pub list: &'a Vec<PlayerAndCharacter>,
}

#[derive(Queryable, Serialize)]
pub struct PlayerAndCharacter {
    pub user_id: i32,
    pub player_name: String,
    pub character_id: Option<i32>,
    pub character_name: Option<String>,
}

/* Structs for handling characters and their creation/modification. */

#[derive(Clone, Debug, Identifiable, Queryable, Serialize)]
#[table_name = "characters"]
pub struct Character {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub abbr: String,
    pub birth_id: i32,
    pub sib_id: i32,
    pub father_id: i32,
    pub father_title_id: Option<i32>,
    pub sl: i32,
    pub crowns: i32,
    pub allowance: i32,
    pub strength: i32,
    pub expertise: i32,
    pub constitution: i32,
    pub endurance: i32,
    pub military_ability: i32,
    pub club_id: Option<i32>,
    pub mistress_id: Option<i32>,
    pub rank_id: Option<i32>,
    pub brevet_rank_id: Option<i32>,
    pub title_id: Option<i32>,
    pub turn_enlisted: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "characters"]
pub struct NewCharacter {
    pub user_id: i32,
    pub name: String,
    pub abbr: String,
    pub birth_id: i32,
    pub sib_id: i32,
    pub father_id: i32,
    pub father_title_id: Option<i32>,
    pub sl: i32,
    pub crowns: i32,
    pub allowance: i32,
    pub strength: i32,
    pub expertise: i32,
    pub constitution: i32,
    pub endurance: i32,
    pub military_ability: i32,
    pub club_id: Option<i32>,
    pub mistress_id: Option<i32>,
    pub rank_id: Option<i32>,
    pub brevet_rank_id: Option<i32>,
    pub title_id: Option<i32>,
    pub turn_enlisted: Option<i32>,
}

#[derive(FromForm)]
pub struct NewPlayerCharacterForm {
    pub player_id: i32,
    pub name: String,
    pub abbr: String,
    pub birth_roll: i32,
    pub sib_roll: i32,
    pub orphan_roll: i32,
    pub father_roll: i32,
    pub title_roll: i32,
    pub strength: i32,
    pub expertise: i32,
    pub constitution: i32,
    pub military_ability: i32,
}

/* Structs for handling rollable tables. */

#[derive(Clone, Debug, Identifiable, Queryable, Serialize)]
#[table_name = "birth_classes"]
pub struct BirthClass {
    pub id: i32,
    pub class_name: String,
}

#[derive(Clone, Debug, Identifiable, Queryable, Serialize)]
#[table_name = "father_positions"]
pub struct FatherPosition {
    pub id: i32,
    pub position_name: String,
    pub initial_funds: i32,
    pub allowance: i32,
    pub inheritance: i32,
    pub starting_sl: i32,
}

#[derive(Clone, Debug, Identifiable, Queryable, Serialize)]
#[table_name = "sibling_ranks"]
pub struct SiblingRank {
    pub id: i32,
    pub rank_name: String,
    pub sl_mod: i32,
    pub funds_multiplier: f32,
    pub allowance_multiplier: f32,
    pub inheritance_multiplier: f32,
}

#[derive(Clone, Debug, Identifiable, Queryable, Serialize)]
#[table_name = "titles"]
pub struct Title {
    pub id: i32,
    pub title_name: String,
    pub starting_sl: i32,
}

// enum Month {
//     January,
//     February,
//     March,
//     April,
//     May,
//     June,
//     July,
//     August,
//     September,
//     October,
//     November,
//     December,
// }

// struct Turn {
//     id: i32,
//     month: Month,
//     year: i32,
// }

// struct Action {
//     id: i32,
//     character_id: i32,
//     turn_id: i32,
//     week: i32,
//     summary: String,
//     detail: String,
//     sp_change: i32,
//     crowns_change: i32,
//     result_detail: String,
// }

// struct Event {
//     id: i32,
//     start_turn_id: i32,
//     end_turn_id: i32,
//     title: String,
//     detail: String,
//     result: String,
// }

// struct Result {
//     id: i32,
//     character_id: i32,
//     turn_id: i32,
//     event_id: i32,
//     summary: String,
//     detail: String,
//     sp_change: i32,
//     crowns_change: i32,
// }

// struct Report {
//     id: i32,
//     turn_id: i32,
//     report_detail: String,
// }

// struct Training {
//     id: i32,
//     character_id: i32,
//     weapon: String,
//     progress: i32,
//     expertise: i32,
// }

// struct Club {
//     id: u64,
//     name: String,
//     rank: i32,
//     status_bonus: i32,
//     dues: i32,
//     requirements: String,
//     gamble_min: i32,
//     gamble_max: i32,
//     gamble_div: i32,
// }

// struct Mistress {
//     id: u64,
//     name: String,
//     sl: i32,
//     beautiful: bool,
//     influential: bool,
//     wealthy: bool,
// }

// struct Brigade {
//     id: u64,
//     name: String,
//     is_cavalry: bool,
//     status_pts: i32
// }

// struct Regiment {
//     id: u64,
//     brigade_id: u64,
//     name: String,
//     rank: i32,
//     weapon: String,
//     friend_id: u64,
//     enemy_id: u64,
//     regiment_commander_id: u64
//     battalion1_commander_id: u64
//     battalion2_commander_id: u64
//     battalion3_commander_id: u64
//     companya_commander_id: u64
//     companyb_commander_id: u64
//     companyc_commander_id: u64
//     companyd_commander_id: u64
//     companye_commander_id: u64
//     companyf_commander_id: u64
// }

// struct Rank {
//     id: u64,
//     regiment_id: u64
//     min_sl: i32,
//     purchase_price: i32,
//     salary: i32,
//     monthly_status: i32,
// }

// struct Mention {
//     id: u64,
//     character_id: u64,
//     turn_mentioned: u64,
//     initial_bonus: i32,
//     notes: String,
// }
