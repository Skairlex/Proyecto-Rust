
use regex::Regex;


 
//===========================================      EMAIL     =========================================
//======================================================================================================




// No admite espacios en blanco
 pub fn ema_no_espacios(c:String)->bool{

    let arr=c.split_whitespace();
    let b=arr.count();
    if b>1{
       return false;
    }
    return true;
}

pub fn ema_no_espacios_inicio_final(c:String)->bool{

    let espacios_inicio  = Regex::new(r"^[^-\s][\s\S]+$").unwrap();
    let espacios_final = Regex::new(r"^[\s\S]+[^-\s]$").unwrap();
    if !espacios_inicio.is_match(&c) || !espacios_final.is_match(&c) {
       return  false;
    }
    return true;
}

// No debe tener el punto antes ni después del @
pub fn ema_validar_punto(c:String)->bool{
    let puntodespues = Regex::new(r"^([\s\S])+@\.+[\s\S]*$").unwrap();
    let puntoantes = Regex::new(r"^([\s\S])+\.@+[\s\S]*$").unwrap();
    if puntodespues.is_match(&c) || puntoantes.is_match(&c) {
       return  false;
    }
    return true;
}


// Se valida el tamaño de la última palabra del dominio, debe
// tener un tamaño mínimo de 2 y máximo 6 letras
pub fn ema_dominio(c:String)->bool{
    let palabra=c.split(".") ;
    let mut ultima="";
    for i in palabra{
        ultima=i;
    }
    let numletras = Regex::new(r"^[a-zA-Z]{2,6}$").unwrap();

    if numletras.is_match(&ultima){
        return true;
    }
    return  false;
}


// Se admiten los siguientes caracteres especiales antes del @
// El punto .
// El guión -
// El guión bajo _
pub fn ema_caracteres(c:String)->bool{

    let caracter = Regex::new(r"^[\.a-zA-Z0-9_-]+@+[\s\S]*$").unwrap();
    if caracter.is_match(&c){
        return true;
    }
    return  false;
}

