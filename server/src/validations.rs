use models::StructPersona;
use regex::Regex;
use crate:: models;


fn validar()->StructPersona{
let per1 = models::StructPersona {
    identificacion: String::from("mi identificacion"),
    nombre: String::from("Gravity's Rainbow"),
    genero: String::from("Gravity's Rainbow"),
    estadocivil:String::from("Gravity's Rainbow"),
    fechanacimiento: String::from("Gravity's Rainbow"), 
    telefono: String::from("Gravity's Rainbow"),
    direccion: String::from("Gravity's Rainbow"),
    email: String::from("Gravity's Rainbow"),
    validado: true,
    observacion: String::from("Gravity's Rainbow"),
};
return per1;
}

pub fn  imprimir(){
    /* 
    let prueba=validar();
    println!("{}",prueba.identificacion);//Campo a revisar
    println!("{}",validar().identificacion);//validado
    println!("{}",validar().identificacion);//registro de observaciones
    */
    println!("{}",dos_digitos_validos("12".to_string()));
}

fn cedula_solo_digitos(c:String)->bool{
    let prob =Regex::new(r"^([0-9]){0,10}$").unwrap();
    if prob.is_match(&c){
       return true; 
    }else {
        return false;
    }

}
fn dos_digitos_validos(c:String)->bool{
    let prob =Regex::new(r"^([01-24]){1,2}$").unwrap();
    if prob.is_match(&c){
       return true; 
    }else {
        return false;
    }

}


fn validarCedula2(per:StructPersona)->StructPersona{
    let ced=per.identificacion;
    let obs=per.observacion;
    let solonum=Regex::new(r"^[ÑA-Z0-9]+$").unwrap();
   
 

    return validar();
}


/* 
fn validarcedula1(cedula: String) -> bool{
    let digitos: Vec<char> = cedula.chars().collect();
    let mut provincia: String = String::new();
    let mut digitosnumeros: Vec<u32> = vec![];
    let mut i = 0;
    let mut validador: bool = true;

    for c in digitos.clone(){
        if c.is_alphabetic() {
            validador = false;
            break;
        } else {
            if i < 2 {
                provincia.push(c);
            }
            digitosnumeros.push(c.to_digit(10).unwrap());
            i+=1;
        }
    }
}
*/

//Se requiere como mínimo que un registro de persona tenga válido un nombre y apellido, un teléfono de contacto y un email.