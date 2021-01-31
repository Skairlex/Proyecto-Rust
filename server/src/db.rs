use crate::{diesel::prelude::*, model::StructPersona};
use dotenv::dotenv;
use std::env;
use diesel::insert_into;
use std::error::Error;



pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[allow(dead_code)]
pub fn insert_persona(conn: &PgConnection, per: StructPersona) -> QueryResult<usize> {
    use crate::schema::persona::dsl::*;

    insert_into(persona)
        .values((
            //  codigo.eq(3),
            identificacion.eq(per.identificacion.to_string()),
            nombre.eq(per.nombre.to_string()),
            genero.eq(per.genero.to_string()),
            estadocivil.eq(per.estadocivil.to_string()),
            fechanacimiento.eq(per.fechanacimiento.to_string()),
            telefono.eq(per.telefono.to_string()),
            direccion.eq(per.direccion.to_string()),
            email.eq(per.email.to_string()),
            validado.eq(per.validado),
            observacion.eq(per.observacion.to_string()),
        ))
        .execute(conn)
}

#[allow(unused_must_use)]
pub fn guardaren_csv() -> Result<(), Box<dyn Error>> {
    let connection = establish_connection(); //Se establece conexión

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_path("./tmp/Prueba1.csv")?;


    for result in reader.records() {
        let record = result?;
        
        let per = StructPersona {
            identificacion: record[0].to_string(),
            nombre: record[1].to_string(),
            genero: record[2].to_string(),
            estadocivil: record[3].to_string(),
            fechanacimiento: record[4].to_string(), 
            telefono: record[5].to_string(),
            direccion: record[6].to_string(),
            email: record[7].to_string(),
            validado: true,
            observacion: String::from("N/A"),
        };

        insert_persona(&connection,per); //Se guarda datos
        
    }
    Ok(())
}