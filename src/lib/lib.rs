#![feature(decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;

pub mod db;
pub mod model;
pub mod player;
pub mod schema;
