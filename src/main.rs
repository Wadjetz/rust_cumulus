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
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate r2d2_diesel;
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
mod token;
mod graphql;
mod config;
mod schema;
mod user;
mod users_repository;
mod users_resolvers;
mod server;
mod routes;

mod feeds; 
mod source;
mod source_type;
mod source_option;
mod sources;
mod sources_repository;
mod sources_resolvers;
mod users_feeds; 
mod users_sources; 
mod mercury; 
mod rss; 

embed_migrations!("migrations");

fn main() {
    server::run();
}
