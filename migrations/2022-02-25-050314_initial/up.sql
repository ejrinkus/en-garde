-- Your SQL goes here
CREATE TABLE players (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE characters (
    id INTEGER PRIMARY KEY NOT NULL,
    /* id of the player that owns this character. */
    player_id INTEGER NOT NULL,
    /* character's name and their name's abbreviation (abbr is optional). */
    name TEXT NOT NULL,
    abbr TEXT NOT NULL,
    /* family info. */
    birth TEXT NOT NULL,
    sib_rank TEXT NOT NULL,
    father TEXT NOT NULL,
    father_title TEXT NOT NULL,
    allowance INTEGER NOT NULL,
    /* status level. */
    sl INTEGER NOT NULL,
    /* current money. */
    crowns INTEGER NOT NULL,
    /* character stats. */
    strength INTEGER NOT NULL,
    expertise INTEGER NOT NULL,
    constitution INTEGER NOT NULL,
    endurance INTEGER NOT NULL,
    military_ability INTEGER NOT NULL,
    /* character's current club (optional). */
    club_id INTEGER,
    /* character's current mistress (optional). */
    mistress_id INTEGER,
    /* character's military rank in their current regiment (optional). Note that
     * each row in the rank table is linked to a specific regiment. So if a
     * character's rank is for a specific regiment and brigade, the rank_id is
     * sufficient to identify those details.
     */
    rank_id INTEGER,
    brevet_rank_id INTEGER,
    /* turn that the player joined their current regiment (required iff
     * regiment_id is not null).
     */
    turn_enlisted INTEGER,
    FOREIGN KEY(player_id) REFERENCES players(id),
    FOREIGN KEY(club_id) REFERENCES clubs(id),
    FOREIGN KEY(mistress_id) REFERENCES mistresses(id),
    FOREIGN KEY(rank_id) REFERENCES ranks(id),
    FOREIGN KEY(brevet_rank_id) REFERENCES ranks(id),
    FOREIGN KEY(turn_enlisted) REFERENCES turns(id)
);

CREATE TABLE turns (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    month INTEGER NOT NULL,
    year INTEGER NOT NULL
);

CREATE TABLE actions (
    id INTEGER PRIMARY KEY NOT NULL,
    /* character performing the action. */
    character_id INTEGER NOT NULL,
    /* turn the action occurred. */
    turn_id INTEGER NOT NULL,
    /* week number the action was taken in. value should be between 1-4. */
    week INTEGER NOT NULL,
    /* summary of the action, and detailed description. */
    summary TEXT NOT NULL,
    detail TEXT NOT NULL,
    /* SP and crowns earned or lost from the action (optional). */
    sp_change INTEGER,
    crowns_change INTEGER,
    /* details of the result of the action. */
    result_detail TEXT,
    FOREIGN KEY(character_id) REFERENCES characters(id),
    FOREIGN KEY(turn_id) REFERENCES turns(id)
);

CREATE TABLE events (
    id INTEGER PRIMARY KEY NOT NULL,
    /* turns the even started/ended. */
    start_turn_id INTEGER NOT NULL,
    end_turn_id INTEGER NOT NULL,
    /* descriptive info about the event. */
    title TEXT NOT NULL,
    detail TEXT NOT NULL,
    /* end result of the even (optional, can be filled in after event ends) */
    result TEXT,
    FOREIGN KEY(start_turn_id) REFERENCES turns(id),
    FOREIGN KEY(end_turn_id) REFERENCES turns(id)
);

CREATE TABLE results (
    id INTEGER PRIMARY KEY NOT NULL,
    /* character the results apply to. */
    character_id INTEGER NOT NULL,
    /* turn the results are associated with (optional). */
    turn_id INTEGER,
    /* event the results are associated with (optional). */
    event_id INTEGER,
    /* summary and detailed description of the results. */
    summary TEXT NOT NULL,
    detail TEXT,
    /* SP/crowns gained/lost from this result. */
    sp_change INTEGER,
    crowns_change INTEGER,
    FOREIGN KEY(character_id) REFERENCES characters(id),
    FOREIGN KEY(turn_id) REFERENCES turns(id),
    FOREIGN KEY(event_id) REFERENCES events(id)
);

CREATE TABLE reports (
    id INTEGER PRIMARY KEY NOT NULL,
    turn_id INTEGER NOT NULL,
    report_detail TEXT NOT NULL,
    FOREIGN KEY(turn_id) REFERENCES turns(id)
);

CREATE TABLE trainings (
    id INTEGER PRIMARY KEY NOT NULL,
    character_id INTEGER NOT NULL,
    weapon TEXT NOT NULL,
    progress INTEGER NOT NULL,
    expertise INTEGER NOT NULL,
    FOREIGN KEY(character_id) REFERENCES characters(id)
);

CREATE TABLE clubs (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    rank INTEGER NOT NULL,
    status_bonus INTEGER NOT NULL,
    dues INTEGER NOT NULL,
    requirements TEXT NOT NULL,
    gamble_min INTEGER NOT NULL,
    gamble_max INTEGER NOT NULL,
    gamble_div INTEGER NOT NULL
);

CREATE TABLE mistresses (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    sl INTEGER NOT NULL,
    beautiful INTEGER NOT NULL,
    influential INTEGER NOT NULL,
    wealthy INTEGER NOT NULL
);

CREATE TABLE brigades (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    is_cavalry INTEGER NOT NULL,
    status_pts INTEGER NOT NULL
);

CREATE TABLE regiments (
    id INTEGER PRIMARY KEY NOT NULL,
    brigade_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    rank INTEGER NOT NULL,
    weapon TEXT NOT NULL,
    friend_id INTEGER,
    enemy_id INTEGER,
    regiment_commander_id INTEGER,
    battalion1_commander_id INTEGER,
    battalion2_commander_id INTEGER,
    battalion3_commander_id INTEGER,
    companya_commander_id INTEGER,
    companyb_commander_id INTEGER,
    companyc_commander_id INTEGER,
    companyd_commander_id INTEGER,
    companye_commander_id INTEGER,
    companyf_commander_id INTEGER,
    FOREIGN KEY(brigade_id) REFERENCES brigades(id),
    FOREIGN KEY(friend_id) REFERENCES regiments(id),
    FOREIGN KEY(enemy_id) REFERENCES regiments(id),
    FOREIGN KEY(regiment_commander_id) REFERENCES characters(id),
    FOREIGN KEY(battalion1_commander_id) REFERENCES characters(id),
    FOREIGN KEY(battalion2_commander_id) REFERENCES characters(id),
    FOREIGN KEY(battalion3_commander_id) REFERENCES characters(id),
    FOREIGN KEY(companya_commander_id) REFERENCES characters(id),
    FOREIGN KEY(companyb_commander_id) REFERENCES characters(id),
    FOREIGN KEY(companyc_commander_id) REFERENCES characters(id),
    FOREIGN KEY(companyd_commander_id) REFERENCES characters(id),
    FOREIGN KEY(companye_commander_id) REFERENCES characters(id),
    FOREIGN KEY(companyf_commander_id) REFERENCES characters(id)
);

CREATE TABLE ranks (
    id INTEGER PRIMARY KEY NOT NULL,
    regiment_id INTEGER,
    min_sl INTEGER,
    purchase_price INTEGER,
    salary INTEGER NOT NULL,
    monthly_status INTEGER NOT NULL,
    FOREIGN KEY(regiment_id) REFERENCES regiments(id)
);

CREATE TABLE mentions (
    id INTEGER PRIMARY KEY NOT NULL,
    character_id INTEGER NOT NULL,
    turn_mentioned INTEGER NOT NULL,
    initial_bonus INTEGER NOT NULL,
    notes TEXT NOT NULL,
    FOREIGN KEY(character_id) REFERENCES characters(id),
    FOREIGN KEY(turn_mentioned) REFERENCES turns(id)
);