
//=====================================   GENERO    ====================================================
//======================================================================================================




// Para ser válido debe estar dentro de los estipulados por el Registro
// Civil de la siguiente manera:

// M = Masculino
// F = Femenino
// NULL

pub fn gen_validar_genero(c:String)->bool{
    if 
    c != "M" &&
    c != "F" &&
    c != "NULL" {
     return  false;}
     return true;
 }