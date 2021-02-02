
 use regex::Regex;
 use substring::Substring;


 //======================================       TELEFONO      =======================================
//======================================================================================================



//-----------------------CCONVENCIONAL---------------------------
//---------------------------------------------------------------





// Debe contener sólo números
// Una longitud de 9 dígitos si es ECUADOR
pub fn con_digitos_tamnio(c:String)->bool{
    let prob =Regex::new(r"^([0-9]){9,9}$").unwrap();
    if prob.is_match(&c){
       return true; 
    }else {
        return false;
    }

}

// Debe empezar con los códigos válidos por provincia, los 2 primeros numero deben estar 
//entre 02-07 si el país es ECUADOR
pub fn con_provincia(c:String)->bool{
    let digito=c.substring(0, 2).parse::<i32>().unwrap();
    if digito>=2 && digito<=7{
        return  true;
    }
  return  false;

}

// El valor por defecto del código de país será 593

// Una longitud mínima de 6 dígitos si es DIFERENTE de
// Ecuador




//-----------------------   CELULAR   ---------------------------
//---------------------------------------------------------------


// Debe contener sólo números
// Tener una longitud de 10 dígitos
pub fn cel_digitos_tamnio(c:String)->bool{
    let prob =Regex::new(r"^([0-9]){10,10}$").unwrap();
    if prob.is_match(&c){
       return true; 
    }else {
        return false;
    }

}


// Empezar con '09'
pub fn cel_primerosdigitos(c:String)->bool{
    if c.substring(0, 2)=="09"{
        return  true;
    }
  return  false;

}




