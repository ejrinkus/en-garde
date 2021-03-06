#![feature(decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod db;
pub mod manipulations;
pub mod model;
pub mod schema;
