#![allow(unused_assignments)]
use regex::Regex;
use chrono::{ Datelike, Utc};


//====================================== FECHA DE NACIMIENTO  =======================================
//======================================================================================================


// La fecha de nacimiento debe estar dentro de un rango de edad de 8 a 95 años.
pub fn fec_rango_edad(c:String)->bool{
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
  
    for i in fecha{
        cont=cont+1;
        if cont==1{nac_anio=i.parse::<i32>().unwrap()}
        if cont==2{nac_mes=i.parse::<i32>().unwrap()}
        if cont==3{nac_dia=i.parse::<i32>().unwrap()}
    }
    edad=act_anio-nac_anio;
    if act_mes<nac_mes{
        edad=edad-1;
    }
    if act_mes==nac_mes && act_dia<=nac_dia{
        edad=edad-1;
    }
          
          if edad>=8 &&edad<=95{
              return  true;
          }
      
   return false;
   }



//  Formato de fecha: yyyy-MM-dd
pub fn fec_formato_fecha(c:String)->bool{
    let refecha = Regex::new(r"^\d{4}\-(0?[1-9]|1[012])\-(0?[1-9]|[12][0-9]|3[01])$").unwrap();
    if refecha.is_match(&c) {
       return true;
    } 
    return false;
   }
   
   