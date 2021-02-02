
use regex::Regex;
use substring::Substring;


//======================================NUMERO DE IDENTIFICACION=======================================
//======================================================================================================



//-----------------------CEDULA---------------------------
//--------------------------------------------------------

//Debe contener sólo dígitos (10) del 0-9.
pub fn ced_solo_digitos(c:String)->bool{
    let prob =Regex::new(r"^([0-9]){10,10}$").unwrap();
    if prob.is_match(&c){
       return true; 
    }else {
        return false;
    }

}

//Debe empezar con dos dígitos válidos 01-24 (24 provincias del Ecuador), 30, 50, 80
pub fn ced_dos_digitos_validos(c:String)->bool{
    //let prob =Regex::new(r"^([0-9]){1,2}$").unwrap();
    //"foobar".substring(2,5), "oba"
    //let prob2=assert!((c.substring(0, 1)),);
    let dos_char=c.substring(0, 2).parse::<i32>().unwrap();
    if dos_char>0 && dos_char<=24 {return true;}
    if dos_char==30 || dos_char==50 || dos_char==80 { return true;}
    return  false;

}

//Debe cumplir con la validación del Dígito Verificador.
pub fn ced_digito_verificador(c:String)->bool{
    
    let mut cambio=true;
    let mut digito;
    let mut suma=0;
    let  residuo;
    let verificador;
    for i in 1..10 {
        if cambio {
            
            digito=c.substring(i-1, i).parse::<i32>().unwrap();
            digito=digito*2;
            if digito>9{digito=digito-9}

         }
        else {
           
            digito=c.substring(i-1, i).parse::<i32>().unwrap();
        }
        cambio=!cambio;
        suma=suma+digito;
     


        
    }

    residuo=suma%10;
    
    if residuo==0{
        verificador=residuo;
    }
    else{
        verificador=10-residuo; 
    }

    if verificador==c.substring(9, 10).parse::<i32>().unwrap(){ return true}
    return  false;

}


//Se va a considerar como válido el número 9999999999
pub fn ced_consumidorfinal(c:String)->bool{
    if c=="9999999999"{return true;}
    return false;
}






//-----------------------RUC---------------------------
//-------------------------------------------------------

//Debe contener sólo dígitos (13) del 0-9.
pub fn ruc_solo_digitos(c:String)->bool{
    let prob =Regex::new(r"^([0-9]){13,13}$").unwrap();
    if prob.is_match(&c){
       return true; 
    }else {
        return false;
    }

}

//Debe empezar con dos dígitos válidos 01-24 los cuales están especificados por el SRI.
pub fn ruc_dos_digitos_validos_ruc(c:String)->bool{
    let dos_char=c.substring(0, 2).parse::<i32>().unwrap();
    if dos_char>0 && dos_char<=24 {return true;}
    return  false;

}





#[allow(unused_assignments)]
//Debe cumplir con la validación del Dígito Verificador.
pub fn ruc_digito_verificador(c:String)->bool{
    let ter_dig=c.substring(2, 3).parse::<i32>().unwrap();
    let  residuo;
    let mut digito;
    let mut suma=0;
    let ter_priv=9;//Tercer digito en privadas
    let ter_pub=6;//Tercer digito en publicas
    let privadas = [4,3,2,7,6,5,4,3,2];
    let publicas = [3,2,7,6,5,4,3,2,-1];
    let array;
    let verificador;//numero verificador resultante
    let mut iteraciones=0;
    if ter_dig==ter_priv {
        iteraciones=9;
        array=privadas;
    }else  if ter_dig==ter_pub {
        iteraciones=8;
        array=publicas;
            } else {
              return false;
            }
    for i in 0..iteraciones {
            digito=c.substring(i, i+1).parse::<i32>().unwrap();
            digito=digito*array[i];
        suma=suma+digito;

    }

    residuo=suma%11;
  
    if residuo==0{
        verificador=residuo;
    }
    else{
        verificador=11-residuo;
    }

    if verificador==c.substring(9, 10).parse::<i32>().unwrap(){ return true}
    //Falta el ultimo digito*/
    return  false;
}

//Los 3 últimos dígitos no pueden ser 000.
pub fn ruc_ult_digitos(c:String)->bool{
    let ultima = Regex::new(r"^([\s\S])+000").unwrap();
    
    if ultima.is_match(&c){
        return false;
    }
    return  true;
}



//-----------------------PASAPORTE---------------------------
//-----------------------------------------------------------



//Debe ser de tipo alfanumérico .Se permite únicamente caracteres de
//“A-Z”, “Ñ”, “0-9”
pub fn pas_char_validos(c:String)->bool{
    let prob =Regex::new(r"^[0-9ÑA-Z]+$").unwrap();
    if prob.is_match(&c){
       return true; 
    }else {
        return false;
    }
}


//Longitud mín. de 5 y máx. de 20 caracteres.
pub fn pas_maximo_valido(c:String)->bool{
    let prob =Regex::new(r"^([\s\S]){5,20}$").unwrap();
    if prob.is_match(&c){
       return true; 
    }else {
        return false;
    }
}

//Este es para que no sea solo numeros,ni solo letras
pub fn pas_alfanumerico(c:String)->bool{
    let regla1 =Regex::new(r"^[0-9]+$").unwrap();
    let regla2 =Regex::new(r"^[A-Z]+$").unwrap();
    if regla2.is_match(&c) ||regla1.is_match(&c) {
       return false; 
    }else {
        return true;
    }
}
