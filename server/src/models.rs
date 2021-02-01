
use serde::Deserialize;
use diesel::{Insertable};
use serde;
use crate::schema::persona;

#[derive( Deserialize,Insertable)]
#[table_name = "persona"]
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