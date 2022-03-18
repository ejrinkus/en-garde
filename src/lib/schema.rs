table! {
    actions (id) {
        id -> Integer,
        character_id -> Integer,
        turn_id -> Integer,
        week -> Integer,
        summary -> Text,
        detail -> Text,
        sp_change -> Nullable<Integer>,
        crowns_change -> Nullable<Integer>,
        result_detail -> Nullable<Text>,
    }
}

table! {
    birth_classes (id) {
        id -> Integer,
        class_name -> Text,
    }
}

table! {
    brigades (id) {
        id -> Integer,
        name -> Text,
        is_cavalry -> Integer,
        status_pts -> Integer,
    }
}

table! {
    characters (id) {
        id -> Integer,
        user_id -> Integer,
        name -> Text,
        abbr -> Text,
        birth_id -> Integer,
        sib_id -> Integer,
        father_id -> Integer,
        father_title_id -> Nullable<Integer>,
        sl -> Integer,
        crowns -> Integer,
        allowance -> Integer,
        strength -> Integer,
        expertise -> Integer,
        constitution -> Integer,
        endurance -> Integer,
        military_ability -> Integer,
        club_id -> Nullable<Integer>,
        mistress_id -> Nullable<Integer>,
        rank_id -> Nullable<Integer>,
        brevet_rank_id -> Nullable<Integer>,
        title_id -> Nullable<Integer>,
        turn_enlisted -> Nullable<Integer>,
    }
}

table! {
    clubs (id) {
        id -> Integer,
        name -> Text,
        rank -> Integer,
        status_bonus -> Integer,
        dues -> Integer,
        requirements -> Text,
        gamble_min -> Integer,
        gamble_max -> Integer,
        gamble_div -> Integer,
    }
}

table! {
    events (id) {
        id -> Integer,
        start_turn_id -> Integer,
        end_turn_id -> Integer,
        title -> Text,
        detail -> Text,
        result -> Nullable<Text>,
    }
}

table! {
    father_positions (id) {
        id -> Integer,
        position_name -> Text,
        initial_funds -> Integer,
        allowance -> Integer,
        inheritance -> Integer,
        starting_sl -> Integer,
    }
}

table! {
    mentions (id) {
        id -> Integer,
        character_id -> Integer,
        turn_mentioned -> Integer,
        initial_bonus -> Integer,
        notes -> Text,
    }
}

table! {
    mistresses (id) {
        id -> Integer,
        name -> Text,
        sl -> Integer,
        beautiful -> Integer,
        influential -> Integer,
        wealthy -> Integer,
    }
}

table! {
    ranks (id) {
        id -> Integer,
        regiment_id -> Nullable<Integer>,
        min_sl -> Nullable<Integer>,
        purchase_price -> Nullable<Integer>,
        salary -> Integer,
        monthly_status -> Integer,
    }
}

table! {
    regiments (id) {
        id -> Integer,
        brigade_id -> Integer,
        name -> Text,
        rank -> Integer,
        weapon -> Text,
        friend_id -> Nullable<Integer>,
        enemy_id -> Nullable<Integer>,
        regiment_commander_id -> Nullable<Integer>,
        battalion1_commander_id -> Nullable<Integer>,
        battalion2_commander_id -> Nullable<Integer>,
        battalion3_commander_id -> Nullable<Integer>,
        companya_commander_id -> Nullable<Integer>,
        companyb_commander_id -> Nullable<Integer>,
        companyc_commander_id -> Nullable<Integer>,
        companyd_commander_id -> Nullable<Integer>,
        companye_commander_id -> Nullable<Integer>,
        companyf_commander_id -> Nullable<Integer>,
    }
}

table! {
    reports (id) {
        id -> Integer,
        turn_id -> Integer,
        report_detail -> Text,
    }
}

table! {
    results (id) {
        id -> Integer,
        character_id -> Integer,
        turn_id -> Nullable<Integer>,
        event_id -> Nullable<Integer>,
        summary -> Text,
        detail -> Nullable<Text>,
        sp_change -> Nullable<Integer>,
        crowns_change -> Nullable<Integer>,
    }
}

table! {
    sibling_ranks (id) {
        id -> Integer,
        rank_name -> Text,
        sl_mod -> Integer,
        funds_multiplier -> Float,
        allowance_multiplier -> Float,
        inheritance_multiplier -> Float,
    }
}

table! {
    titles (id) {
        id -> Integer,
        title_name -> Text,
        starting_sl -> Integer,
    }
}

table! {
    trainings (id) {
        id -> Integer,
        character_id -> Integer,
        weapon -> Text,
        progress -> Integer,
        expertise -> Integer,
    }
}

table! {
    turns (id) {
        id -> Integer,
        month -> Integer,
        year -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        permissions_mask -> Nullable<Integer>,
        pwd -> Text,
    }
}

joinable!(actions -> characters (character_id));
joinable!(actions -> turns (turn_id));
joinable!(characters -> birth_classes (birth_id));
joinable!(characters -> clubs (club_id));
joinable!(characters -> father_positions (father_id));
joinable!(characters -> mistresses (mistress_id));
joinable!(characters -> sibling_ranks (sib_id));
joinable!(characters -> turns (turn_enlisted));
joinable!(characters -> users (user_id));
joinable!(mentions -> characters (character_id));
joinable!(mentions -> turns (turn_mentioned));
joinable!(ranks -> regiments (regiment_id));
joinable!(regiments -> brigades (brigade_id));
joinable!(reports -> turns (turn_id));
joinable!(results -> characters (character_id));
joinable!(results -> events (event_id));
joinable!(results -> turns (turn_id));
joinable!(trainings -> characters (character_id));

allow_tables_to_appear_in_same_query!(
    actions,
    birth_classes,
    brigades,
    characters,
    clubs,
    events,
    father_positions,
    mentions,
    mistresses,
    ranks,
    regiments,
    reports,
    results,
    sibling_ranks,
    titles,
    trainings,
    turns,
    users,
);
