
//use dbo_operations::save_csv_in_struct;


#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate regex;

mod actix_service;
mod schema;
mod dbo_operations;
mod models;
mod validate;


#[path="./utils_validations/num_identificacion.rs"]
mod num_identificacion;

#[path="./utils_validations/nombre.rs"]
mod nombre;

#[path="./utils_validations/est_civil.rs"]
mod est_civil;

#[path="./utils_validations/genero.rs"]
mod genero;

#[path="./utils_validations/direccion.rs"]
mod direccion;

#[path="./utils_validations/email.rs"]
mod email;

#[path="./utils_validations/fecha_nac.rs"]
mod fecha_nac;

#[path="./utils_validations/telefono.rs"]
mod telefono;



#[allow(unused_must_use)]
fn main() {
   actix_service::turn_on();//Activa el servicio multipart aún sin 
   //save_csv_in_struct();  //
   //validate::imprimir();
}
