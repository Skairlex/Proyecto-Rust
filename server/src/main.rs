
use dbo_operations::save_csv_in_struct;


#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate regex;

mod actix_service;
mod schema;
mod dbo_operations;
mod models;
mod validations;

#[allow(unused_must_use)]
fn main() {
   //actix_service::turn_on();//Activa el servicio multipart aún sin 
   //save_csv_in_struct();  //
   validations::imprimir();
}
