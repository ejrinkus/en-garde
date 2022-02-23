use rocket::serde::Serialize;
use rocket::*;

#[derive(Clone, Debug, FromForm, Serialize)]
pub struct Player {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize)]
pub struct Context {
    pub first_name: String,
    pub last_name: String,
}

// struct Character {
//     id: u64,
//     pid: u64,
//     name: String,
//     abbrev: String,
//     status: u32,
//     strength: u32,
//     expertise: u32,
//     constitution: u32,
//     endurance: u32,
// }

// struct Club {
//     id: u64,
//     name: String,
//     rank: u32,
//     status_bonus: u32,
//     dues: u32,
//     requirements: String,
//     gamble_min: u32,
//     gamble_max: u32,
//     gamble_div: u32,
// }

// struct Mistress {
//     id: u64,
//     name: String,
//     status: u32,
//     suitor_id: u64,
//     beautiful: bool,
//     influential: bool,
//     wealthy: bool,
// }

// struct RankDetail {
//     min_level: u32,
//     purchase_price: u32,
//     salary: u32,
//     monthly_status: u32,
// }

// struct Regiment {
//     id: u64,
//     name: String,
//     rank: u32,
//     weapon: String,
//     friend_id: u64,
//     enemy_id: u64,
//     private_detail: RankDetail,
//     subaltern_detail: RankDetail,
//     captain_detail: RankDetail,
//     major_detail: RankDetail,
//     lieutenant_detail: RankDetail,
//     colonel_detail: RankDetail,
// }
