use rocket::serde::Serialize;
use rocket::*;

#[derive(Clone, Debug, FromForm, Queryable, Serialize)]
pub struct Player {
    pub id: i32,
    pub name: String,
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

// struct Character {
//     id: i32,
//     player_id: i32,
//     name: String,
//     abbr: String,
//     sl: i32,
//     strength: i32,
//     expertise: i32,
//     constitution: i32,
//     endurance: i32,
//     military_ability: i32,
//     club_id: i32,
//     mistress_id: i32,
//     rank_id: i32,
//     turn_enlisted: i32,
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
