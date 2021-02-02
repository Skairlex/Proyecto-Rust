


//====================================== ESTADO CIVIL ===================================================
//======================================================================================================


// Se almacenará el estado civil de acuerdo al Diccionario del Registro Civil:

// SOLTERO
// CASADO
// DIVORCIADO
// VIUDO
// EN UNION DE HECHO
// NULL

 pub fn est_civil_validar(c:String)->bool{
    if 
    c != "SOLTERO" &&
    c != "CASADO" &&
    c != "DIVORCIADO" &&
    c != "VIUDO" &&
    c != "EN UNION DE HECHO" &&
    c != "NULL" {
     return  false;}
     return true;
 }


 // Se debe inactivar en estado civil „UNION LIBRE y SEPARADO‟
 pub fn est_civil_estados_inactivos(c:String)->bool{
    if 
    c != "UNION LIBRE" &&
    c != "SEPARADO" {
     return  false;}
     return true;
 }
