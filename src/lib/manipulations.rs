use crate::db::*;
use crate::model::*;

// Returns a tuple consisting of (starting sl, starting funds, allowance).
pub fn get_starting_resources(
    rank_id: i32,
    position_id: i32,
    maybe_title: Option<i32>,
) -> (i32, i32, i32) {
    // Get the data from the relevant rollable tables.
    let rank: SiblingRank = get_sib_rank(rank_id);
    let position: FatherPosition = get_father_pos(position_id);

    // Initial funds are calculated based on the father's initial funds/inheritance,
    // and sibling rank.
    let funds = (position.initial_funds as f32 * rank.funds_multiplier)
        + (position.inheritance as f32 * rank.inheritance_multiplier);

    // Initial allowance comes from the father, and is modified by sibling rank.
    let allowance = position.allowance as f32 * rank.allowance_multiplier;

    // Initial sl is based on the father's position (or title, if they have one),
    // and modified by sibling rank.
    let sl = match maybe_title {
        Some(tid) => get_title(tid).starting_sl,
        None => position.starting_sl,
    } + rank.sl_mod;

    (sl, funds.ceil() as i32, allowance.ceil() as i32)
}
