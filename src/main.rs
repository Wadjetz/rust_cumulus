#![recursion_limit = "1024"]
#![feature(plugin, custom_attribute, custom_derive, associated_type_defaults)]
#![plugin(rocket_codegen)]
#![allow(unused_doc_comment)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate dotenv;
extern crate jsonwebtoken;
extern crate bcrypt;
extern crate crypto;
extern crate uuid;
extern crate chrono;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate postgres;
#[macro_use]
extern crate postgres_derive;
extern crate r2d2;
extern crate r2d2_postgres;
// extern crate r2d2_diesel;
extern crate diesel;
extern crate strum;
#[macro_use]
extern crate strum_macros;
extern crate feed_rs;
#[macro_use]
extern crate hyper;
extern crate reqwest;
extern crate rocket;
extern crate url;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate juniper_codegen;
extern crate juniper_rocket;
#[macro_use] 
extern crate validator_derive;
extern crate validator;

mod errors;
mod pg;
mod migration;
mod migrations;
mod token;
mod graphql;
mod config;
mod mindstream;
mod users;
mod server;
mod routes;

fn main() {
    server::run();
}
