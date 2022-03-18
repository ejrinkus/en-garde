-- Your SQL goes here

/* Users table representing the users and GM for the game. */
CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    /* For now, only 2 bits of info are stored here.  First (most significant) bit is GM priviledges
     * and second bit is admin priviledges.  A zero or null value is a regular player.
     */
    permissions_mask INTEGER,
    /* Yes, passwords are currently stored in clear text on the DB.  Yes, that means you should
     * absolutely _not_ reuse another password here unless you want it to be put at significant risk
     * of it being stolen.  Yes, I'd like put in a real password system and this is just a
     * placeholder for now.  No, it isn't _actually_ protecting any truly sensitive information so
     * it really isn't a big deal for now.
     */
     pwd TEXT NOT NULL
);

/* ROLLABLE TABLES
 *
 * These tables represent the various rollable tables for En Guarde.  They currently are intended to
 * be immutable, although customization may be added at a later date.  ID fields usually correspond
 * to the die roll column on the table (when appropriate).
 */
CREATE TABLE birth_classes (
    id INTEGER PRIMARY KEY NOT NULL,
    class_name TEXT NOT NULL
);
INSERT INTO birth_classes (id, class_name)
VALUES
    (1, "Commoner"),
    (2, "Commoner"),
    (3, "Gentleman"),
    (4, "Gentleman"),
    (5, "Nobleman"),
    (6, "Nobleman");

CREATE TABLE sibling_ranks (
    id INTEGER PRIMARY KEY NOT NULL,
    rank_name TEXT NOT NULL,
    sl_mod INTEGER NOT NULL,
    funds_multiplier REAL NOT NULL,
    allowance_multiplier REAL NOT NULL,
    inheritance_multiplier REAL NOT NULL
);
INSERT INTO sibling_ranks (id, rank_name, sl_mod, funds_multiplier, allowance_multiplier, inheritance_multiplier)
VALUES
    (0, "First Son (Orphan)", 1, 1.1, 0.0, 1.0),
    (1, "First Son", 1, 1.1, 1.1, 0.0),
    (2, "Second Son", 0, 1.0, 1.0, 0.0),
    (3, "Second Son", 0, 1.0, 1.0, 0.0),
    (4, "Second Son", 0, 1.0, 1.0, 0.0),
    (5, "Bastard", -1, 0.9, 0.9, 0.0),
    (6, "Bastard", -1, 0.9, 0.9, 0.0);

CREATE TABLE father_positions (
    id INTEGER PRIMARY KEY NOT NULL,
    position_name TEXT NOT NULL,
    initial_funds INTEGER NOT NULL,
    allowance INTEGER NOT NULL,
    inheritance INTEGER NOT NULL,
    starting_sl INTEGER NOT NULL
);
INSERT INTO father_positions (id, position_name, initial_funds, allowance, inheritance, starting_sl)
VALUES
    /* Commoner Positions*/
    (1, "Peasant", 10, 0, 0, 2),
    (2, "Peasant", 10, 0, 0, 2),
    (3, "Small Merchant", 25, 5, 100, 3),
    (4, "Merchant", 150, 20, 750, 3),
    (5, "Wealthy Merchant", 250, 50, 1500, 3),
    (6, "Very Wealthy Merchant", 500, 100, 4000, 3),
    /* Gentleman Positions*/
    (7, "Impoverished", 40, 0, 100, 4),
    (8, "Impoverished", 40, 0, 100, 4),
    (9, "Well-to-do", 250, 50, 1500, 4),
    (10, "Well-to-do", 250, 50, 1500, 4),
    (11, "Wealthy", 500, 100, 4000, 4),
    (12, "Very Wealthy", 750, 125, 5000, 5),
    /* Noble Positions*/
    (13, "Impoverished", 40, 0, 100, 0),
    (14, "Impoverished", 40, 0, 100, 0),
    (15, "Well-to-do", 250, 50, 1500, 0),
    (16, "Wealthy", 500, 100, 4000, 0),
    (17, "Very Wealthy", 750, 125, 5000, 0),
    (18, "Very Wealthy", 750, 125, 5000, 0);

CREATE TABLE titles (
    id INTEGER PRIMARY KEY NOT NULL,
    title_name TEXT NOT NULL,
    starting_sl INTEGER NOT NULL
);
INSERT INTO titles (id, title_name, starting_sl)
VALUES
    (1, "Knight", 6),
    (2, "Baron", 7),
    (3, "Marquis", 8),
    (4, "Earl", 9),
    (5, "Viscount", 10),
    (6, "Count", 11);

/* Table containing all the characters in the game.  Generally should be player characters, but it
 * should be able to support NPCs as well (NPCs should be assigned to the GM).
 */
CREATE TABLE characters (
    id INTEGER PRIMARY KEY NOT NULL,
    /* id of the player that owns this character. */
    user_id INTEGER NOT NULL,
    /* character's name and their name's abbreviation (abbr is optional). */
    name TEXT NOT NULL,
    abbr TEXT NOT NULL,
    /* family info. */
    birth_id INTEGER NOT NULL,
    sib_id INTEGER NOT NULL,
    father_id INTEGER NOT NULL,
    father_title_id INTEGER,
    /* status level. */
    sl INTEGER NOT NULL,
    /* current money. */
    crowns INTEGER NOT NULL,
    allowance INTEGER NOT NULL,
    /* character stats. */
    strength INTEGER NOT NULL,
    expertise INTEGER NOT NULL,
    constitution INTEGER NOT NULL,
    endurance INTEGER NOT NULL,  /* Current endurance; max endurance is strength * constitution */
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
    title_id INTEGER,
    /* turn that the player joined their current regiment (required iff
     * regiment_id is not null).
     */
    turn_enlisted INTEGER,
    FOREIGN KEY(user_id) REFERENCES users(id),
    FOREIGN KEY(birth_id) REFERENCES birth_classes(id),
    FOREIGN KEY(sib_id) REFERENCES sibling_ranks(id),
    FOREIGN KEY(father_id) REFERENCES father_positions(id),
    FOREIGN KEY(father_title_id) REFERENCES titles(id),
    FOREIGN KEY(club_id) REFERENCES clubs(id),
    FOREIGN KEY(mistress_id) REFERENCES mistresses(id),
    FOREIGN KEY(rank_id) REFERENCES ranks(id),
    FOREIGN KEY(brevet_rank_id) REFERENCES ranks(id),
    FOREIGN kEY(title_id) REFERENCES titles(id),
    FOREIGN KEY(turn_enlisted) REFERENCES turns(id)
);

/* TURN TRACKING
 *
 * Various tables needed to track turns, actions, reports, events, and the passage of time.
 */

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

CREATE TABLE mentions (
    id INTEGER PRIMARY KEY NOT NULL,
    character_id INTEGER NOT NULL,
    turn_mentioned INTEGER NOT NULL,
    initial_bonus INTEGER NOT NULL,
    notes TEXT NOT NULL,
    FOREIGN KEY(character_id) REFERENCES characters(id),
    FOREIGN KEY(turn_mentioned) REFERENCES turns(id)
);

/* FACTIONS
 *
 * Tables tracking various factions and appointments that characters can earn.  Similar to the
 * rollable tables in that they're generally predefined and immutable (with the exception of the
 * mistresses).
 */

CREATE TABLE mistresses (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    sl INTEGER NOT NULL,
    beautiful INTEGER NOT NULL,
    influential INTEGER NOT NULL,
    wealthy INTEGER NOT NULL
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