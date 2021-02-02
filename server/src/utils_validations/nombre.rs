use regex::Regex;




//======================================      NOMBRE        =======================================
//======================================================================================================


//-----------------------PERSONA-----------------------------
//-----------------------------------------------------------

// Debe ser ingresado en mayúsculas.     POR HACER



// No debe contener caracteres especiales.
// Los nombres solo pueden contener caracteres de a A la Z + Ñ.
pub fn per_mayusculas_nombre(c:String)->bool{
    let regla =Regex::new(r"^[A-ZÑ\s]+$").unwrap();
    if regla.is_match(&c)  {
       return true; 
    }else {
        return false;
    }
}


// Las vocales tildadas se guardaran en la base sin tildar.
pub fn per_tildadas(ch:String )->String{
   
    let mut cambiado = "".to_owned();
    let char_vec: Vec<char> = ch.chars().collect();
    let mut letra;
    for c in char_vec {
        if c=='Á'{
            letra='A'.to_string();
        }else if c=='É'{
            letra='E'.to_string();
        }else if c=='Í'{
            letra='I'.to_string();
        }else if c=='Ó'{
            letra='O'.to_string();
        }else if c=='Ú'{
            letra='U'.to_string();
        }else {
            letra=c.to_string();
        }
        cambiado.push_str(&letra);
    }
  
    return  cambiado.to_string();
}


// La longitud mínima para nombre completo debe ser de dos palabras.
pub fn per_nombre_minimo_de_palabras(c:String)->bool{
    let arr=c.split_whitespace();
    let b=arr.count();
    if b>=2{
       return true;

    }
    return false;
}



//-----------------------EMPRESA-----------------------------
//-----------------------------------------------------------


//El nombre de una empresa es considerado válido si posee más de dos caracteres.
pub fn emp_tamanio_empresa(c:String)->bool{
    let prob =Regex::new(r"^([\s\S]){0,2}$").unwrap();
    if !prob.is_match(&c){
       return true; 
    }else {
        return false;
    }
}


// El nombre de una empresa puede contener los siguientes
// caracteres: de 0 a 9, del a A a la Z + la Ñ, & ampersand, -
// guion medio, .punto, ()paréntesis,
// / slash, @arroba, ' comilla simple.
pub fn emp_caracteres_empresa(c:String)->bool{
    let prob =Regex::new(r"^[\.\(\)'@/0-9A-ZÑ&-]*$").unwrap();
    if prob.is_match(&c){
       return true; 
    }else {
        return false;
    }
}
