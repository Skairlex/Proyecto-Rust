
use regex::Regex;
use substring::Substring;
use unidecode::unidecode;
use chrono::{DateTime, Datelike, Utc};







//-------------CEDULA------------------
//HECHO
fn cedula_solo_digitos(c:String)->bool{
    let prob =Regex::new(r"^([0-9]){10,10}$").unwrap();
    if prob.is_match(&c){
       return true; 
    }else {
        return false;
    }

}
//HECHO
fn dos_digitos_validos(c:String)->bool{
    //let prob =Regex::new(r"^([0-9]){1,2}$").unwrap();
    //"foobar".substring(2,5), "oba"
    //let prob2=assert!((c.substring(0, 1)),);
    let dos_char=c.substring(0, 2).parse::<i32>().unwrap();
    if dos_char>0 && dos_char<=24 {return true;}
    if dos_char==30 || dos_char==50 || dos_char==80 { return true;}
    //println!("{}",dos_char);
    return  false;

}
//HECHO
fn digito_verificador(c:String)->bool{
    
    let mut cambio=true;
    let mut digito;
    let mut suma=0;
    let  residuo;
    let verificador;
    for i in 1..10 {
        //println!("{}", i );
        if cambio {
            //println!("{} es impar",i  );
            digito=c.substring(i-1, i).parse::<i32>().unwrap();
            digito=digito*2;
            if digito>9{digito=digito-9}

         }
        else {
            //println!("{} es par",i  );
            digito=c.substring(i-1, i).parse::<i32>().unwrap();
        }
        cambio=!cambio;
        suma=suma+digito;
        println!("{}",suma)


        
    }

    residuo=suma%10;
    println!("{}",residuo);
    if residuo==0{
        verificador=residuo
    }
    else{
        verificador=10-residuo
    }

    if verificador==c.substring(9, 10).parse::<i32>().unwrap(){ return true}
    //Falta el ultimo digito
    return  false;

}



fn consumidorfinal(c:String)->bool{
    if c=="9999999999"{return true;}
    return false;
}

//-------------------PASAPORTE----------------

fn char_validos_pasaporte(c:String)->bool{
    let prob =Regex::new(r"^[0-9ÑA-Z]+$").unwrap();
    if prob.is_match(&c){
       return true; 
    }else {
        return false;
    }
}
fn maximo_valido_pasaporte(c:String)->bool{
    let prob =Regex::new(r"^([\s\S]){5,20}$").unwrap();
    if prob.is_match(&c){
       return true; 
    }else {
        return false;
    }
}
fn alfanumerico_pasaporte(c:String)->bool{
    let regla1 =Regex::new(r"^[0-9]+$").unwrap();
    let regla2 =Regex::new(r"^[A-Z]+$").unwrap();
    if regla2.is_match(&c) ||regla1.is_match(&c) {
       return false; 
    }else {
        return true;
    }
}

//--------------------NOMBRE-------------------

/*
Debe ser ingresado en mayúsculas.///            POR HACER
No debe contener caracteres especiales.
Los nombres solo pueden contener caracteres de a A la Z + Ñ.
*/
fn mayusculas_nombre(c:String)->bool{
    let regla =Regex::new(r"^[A-ZÑ]+$").unwrap();
    if regla.is_match(&c)  {
       return true; 
    }else {
        return false;
    }
}

fn tildadas(ch:String )->String{
   
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
   // println!("{}",cambiado);
    //println!("{}",s);
    return  cambiado.to_string();
}

fn nombre_minimo_de_palabras(c:String)->bool{
    let arr=c.split_whitespace();
    let b=arr.count();
    if b>=2{
       return true;

    }
    return false;
}



//----------------------------------GENERO----------------------------------------


fn validar_genero(c:String)->bool{
    if 
    c != "M" &&
    c != "F" &&
    c != "NULL" {
     return  false;}
     return true;
 }

 //----------------------------------ESTADO CIVIL----------------------------------------

fn validar_est_civil(c:String)->bool{
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

 //----------------------------------ESTADO CIVIL----------------------------------------
 fn estados_inactivos(c:String)->bool{
    if 
    c != "UNION LIBRE" &&
    c != "SEPARADO" {
     return  false;}
     return true;
 }

 //---------------------------------FECHA DE NACIMIENTO----------------------------------------
 fn formato_fecha(c:String)->bool{
 let refecha = Regex::new(r"^\d{4}\-(0?[1-9]|1[012])\-(0?[1-9]|1[0-9]|3[01])$$").unwrap();
 if refecha.is_match(&c) {
    return true;
 } 
 println!("Formato no valido");
 return false;
}


fn rango_edad(c:String)->bool{
    let now = Utc::now();
    let act_anio=now.year();
    let act_mes=now.month() as i32;
    let act_dia=now.day() as i32;
    let fecha=c.split("-") ;
    let mut edad=0;
    let mut cont=0;
    let mut nac_anio=0;
    let mut nac_mes=0;
    let mut nac_dia=0;
   // let aux;
   let mut arr=[1,2,3];
    for i in fecha{
        cont=cont+1;
        if cont==1{nac_anio=i.parse::<i32>().unwrap()}
        if cont==2{nac_mes=i.parse::<i32>().unwrap()}
        if cont==3{nac_dia=i.parse::<i32>().unwrap()}
    }
    edad=act_anio-nac_anio;
    //println!("{}-{}",act_mes,nac_mes);
    if act_mes<nac_mes{
        edad=edad-1;
    }
    if act_mes==nac_mes && act_dia<=nac_dia{
        edad=edad-1;
    }
          //  println!("{}",edad);
          if edad>=8 &&edad<=95{
              return  true;
          }
      
   return false;
   }



 //---------------------------------TELEFONO----------------------------------------
 //CONVECIONAL

 fn telefono_digitos_tamnio(c:String)->bool{
    let prob =Regex::new(r"^([0-9]){9,9}$").unwrap();
    if prob.is_match(&c){
       return true; 
    }else {
        return false;
    }

}

fn telefono_provincia(c:String)->bool{
    let digito=c.substring(0, 2).parse::<i32>().unwrap();
    //println!("{}",digito);
    if digito>=2 && digito<=7{
        return  true;
    }
  return  false;

}

//Hacer que se guarde con 593


//CELULAR
fn celular_digitos_tamnio(c:String)->bool{
    let prob =Regex::new(r"^([0-9]){10,10}$").unwrap();
    if prob.is_match(&c){
       return true; 
    }else {
        return false;
    }

}

fn celular_primerosdigitos(c:String)->bool{
    if c.substring(0, 2)=="09"{
        return  true;
    }
  return  false;

}





 //---------------------------------DIRECCION----------------------------------------
 fn direccion_minimo_de_palabras(c:String)->bool{
    let arr=c.split_whitespace();
    let b=arr.count();
    if b>=2{
       return true;
    }
    return false;
}



 //---------------------------------EMAIL----------------------------------------
 fn no_espacios(c:String)->bool{

    let arr=c.split_whitespace();
    let b=arr.count();
    if b>1{
       return false;
    }
    return true;
}



 fn validar_punto(c:String)->bool{
    let puntodespues = Regex::new(r"^([\s\S])+@\.+[\s\S]*$").unwrap();
    let puntoantes = Regex::new(r"^([\s\S])+\.@+[\s\S]*$").unwrap();
    if puntodespues.is_match(&c) || puntoantes.is_match(&c) {
       return  false;
    }
    return true;
}



fn dominio(c:String)->bool{
    let palabra=c.split(".") ;
    let mut ultima="";
    for i in palabra{
        ultima=i;
    }
    println!("{}",ultima);
    let numletras = Regex::new(r"^[a-zA-Z]{2,6}$").unwrap();
    //println!("{}",ultima.char_indices().count());

    if numletras.is_match(&ultima){
        return true;
    }
    return  false;
}

fn caracteres(c:String)->bool{

    let caracter = Regex::new(r"^[\.a-zA-Z0-9_-]+@+[\s\S]*$").unwrap();
    //println!("{}",ultima.char_indices().count());

    if caracter.is_match(&c){
        return true;
    }
    return  false;
}


pub fn  main(){
    /* 
    let prueba=validar();
    println!("{}",prueba.identificacion);//Campo a revisar
    println!("{}",validar().identificacion);//validado
    println!("{}",validar().identificacion);//registro de observaciones
    */
   
    //assert_eq!(unidecode("げんまい茶"), "genmaiCha ");
    //println!("{}",unidecode("í"));
    //println!("{}",mayusculas_nombre("Ñ".to_string()));
    //println!("{}",tildadas("ÁÉÍÓÚasa".to_string().to_uppercase()));
    //tildadas("ÁÉÍÓÚasa".to_string().to_uppercase());
    println!("{}",caracteres("j_fpo-n.c*e1@espe.edu.eA".to_string()));
    
}