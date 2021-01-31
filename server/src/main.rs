
use db::guardaren_csv;


#[macro_use]
extern crate diesel;
extern crate dotenv;

mod actix_service;
mod schema;
mod db;
mod model;

#[allow(unused_must_use)]
fn main() {
   //actix_service::turn_on();
   guardaren_csv();
}
