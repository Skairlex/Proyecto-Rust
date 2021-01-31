
use serde::Deserialize;
use diesel::{Insertable};
use serde;
use crate::schema::persona;

#[derive( Deserialize)]
pub struct StructPersona {
   pub identificacion:String,
   pub nombre: String,
   pub genero: String,
   pub estadocivil:String,
   pub fechanacimiento: String,
   pub telefono: String,
   pub direccion: String,
   pub email: String,
   pub validado:bool,
   pub observacion: String,
}


#[derive(Deserialize, Insertable)]
#[table_name = "persona"]
pub struct NewPerson<'a> {
    pub identificacion: &'a str,
    pub nombre: &'a str,
    pub genero: &'a str,
    pub estadocivil: &'a str,
    pub fechanacimiento: &'a str,
    pub telefono: &'a str,
    pub direccion: &'a str,
    pub email: &'a str,
    pub observacion: &'a str,
}

