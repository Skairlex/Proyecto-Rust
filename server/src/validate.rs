#![allow(unused_assignments)]
#![allow(unused_variables)]
use models::StructPersona;
use crate:: models;
use crate::direccion::*;
use crate::email::*;
use crate::est_civil::*;
use crate::fecha_nac::*;
use crate::genero::*;
use crate::nombre::*;
use crate::num_identificacion::*;
use crate::telefono::*;




pub fn validar_persona(per:StructPersona)->StructPersona{
    

    let valido_id=per.identificacion;
    let valido_nom=per_tildadas(per.nombre.to_uppercase());
    let valido_gen=per.genero;
    let valido_est=per.estadocivil;
    let valido_fec=per.fechanacimiento;
    let valido_tel=per.telefono;
    let valido_dir=per.direccion;
    let valido_ema=per.email;
    let mut valido_val=false;
    let mut valido_obs=String::new();//Pilas con el validado
    
    


    
    

//----------------------------IDENTIFICACION---------------------------------

    let mut cedula_obs=String::new();
    let mut pasaporte_obs=String::new();
    let mut ruc_obs=String::new();

    let mut estado_id=false;


    //CEDULA
    if estado_id==false{
    if !ced_solo_digitos(valido_id.clone()){
        cedula_obs.push_str("CEDULA: Solo se aceptan 10 digitos. \n");
    }else if !ced_dos_digitos_validos(valido_id.clone()){
        cedula_obs.push_str("CEDULA: Codigos de provincia erroneos. \n");
    }else if !ced_digito_verificador(valido_id.clone()) && !ced_consumidorfinal(valido_id.clone()){
        cedula_obs.push_str("CEDULA: No es una cedula valida. \n");
    }else {
        estado_id=true;   
    }
    }
    //RUC
    if estado_id==false{
    if !ruc_solo_digitos(valido_id.clone()){
        ruc_obs.push_str("RUC: Solo se aceptan 13 digitos. \n");
    }else if !ruc_dos_digitos_validos_ruc(valido_id.clone()){
        ruc_obs.push_str("RUC: Codigos de provincia erroneos. \n");
    }else if !ruc_digito_verificador(valido_id.clone())|| !ruc_ult_digitos(valido_id.clone()){
        ruc_obs.push_str("RUC: No es una cedula valida. \n");
    }else {
        estado_id=true;   
       }
    }

    //PASAPORTE
    if estado_id==false{
    if !pas_maximo_valido(valido_id.clone()){
        pasaporte_obs.push_str("PASAPORTE: Longitud mín. de 5 y máx. de 20 caracteres. \n");
    }else if !pas_alfanumerico(valido_id.clone()){
        pasaporte_obs.push_str("PASAPORTE: Debe ser alfanumerico. \n");
    }else if !pas_char_validos(valido_id.clone()){
        pasaporte_obs.push_str("PASAPORTE: Solo se aceptan numeros y letras. \n");
    }else {
        estado_id=true;   
       }
    }

    if estado_id==false {
        valido_obs.push_str(&cedula_obs);
        valido_obs.push_str(&ruc_obs);
        valido_obs.push_str(&pasaporte_obs);        
    }

//----------------------------NOMBRE---------------------------------

    
    let mut persona_obs=String::new();
    let mut empresa_obs=String::new();
    let mut estado_nom=false;

    //PERSONA
    if estado_nom==false{
    if !per_mayusculas_nombre(valido_nom.clone()){
        persona_obs.push_str("PERSONA: Solo se aceptan letras. \n");
    }else if !per_nombre_minimo_de_palabras(valido_nom.clone()){
        persona_obs.push_str("PERSONA: Debe por lo menos tener dos palabras. \n");
    }else {
        estado_nom=true;   
       }
    }

    //EMPRESA
    if estado_nom==false{
    if !emp_caracteres_empresa(valido_nom.clone()){
        empresa_obs.push_str("EMPRESA: Por lo menos debe tener dos caracteres. \n");
    }else if !emp_tamanio_empresa(valido_nom.clone()){
        empresa_obs.push_str("EMPRESA: Contiene caracteres inválidos. \n");
    }else {
        estado_nom=true;   
       }
    }

    if estado_nom==false {
        valido_obs.push_str(&persona_obs);
        valido_obs.push_str(&empresa_obs);
             
    }


//----------------------------GENERO---------------------------------
    let mut estado_gen=false;

    if !gen_validar_genero(valido_gen.clone()){
        valido_obs.push_str("GENERO: Solo puede ingresar M,F,NULL. \n");
    }else {
        estado_gen=true
    }

//----------------------------ESTADO CIVIL---------------------------------
    let mut estado_est=false;

    if !est_civil_validar(valido_est.clone()){
        valido_obs.push_str("ESTCIVIL: No es una opción valida. \n");
    }else if est_civil_estados_inactivos(valido_est.clone()){
        valido_obs.push_str("ESTCIVIL: Es un estado inactivo. \n");
    }else {
        estado_est=true
    }

//----------------------------FECHA DE NACIMIENTO---------------------------------

    let mut estado_fec=false;

    if !fec_formato_fecha(valido_fec.clone()){
        valido_obs.push_str("FECHA: Debe tener  el formato yyyy-MM-dd. \n");
    }else if !fec_rango_edad(valido_fec.clone()){
        valido_obs.push_str("FECHA: Solo se acepta de 8 a 95 años. \n");
    }else {
        estado_fec=true
    }



//----------------------------TELEFONO---------------------------------
 
    let mut convencional_obs=String::new();
    let mut celular_obs=String::new();
    let mut estado_tel=false;

    //CONVENCIONAL
    if estado_tel==false{
    if !con_digitos_tamnio(valido_tel.clone()){
        convencional_obs.push_str("CONVENCIONAL: Solo se aceptan 9 digitos. \n");
    }else if !con_provincia(valido_tel.clone()){
        convencional_obs.push_str("CONVENCIONAL: Revise los digitos de provincia. \n");
    }else {
        estado_tel=true;   
       }
    }

    //CELULAR
    if estado_tel==false{
    if !cel_digitos_tamnio(valido_tel.clone()){
        celular_obs.push_str("CELULAR: Solo se acpetan 10 digitos. \n");
    }else if !cel_primerosdigitos(valido_tel.clone()){
        celular_obs.push_str("CELULAR: debe comenzar con 09. \n");
    }else {
        estado_tel=true;   
       }
    }

    if estado_tel==false {
        valido_obs.push_str(&convencional_obs);
        valido_obs.push_str(&celular_obs);
             
    }

//----------------------------DIRECCION---------------------------------    


  let mut estado_dir=false;

    if !dir_minimo_de_palabras(valido_dir.clone()){
        valido_obs.push_str("DIRECCION: Debe por lo menos tener dos palabras. \n");
    }else {
        estado_dir=true
    }

//----------------------------EMAIL------------------------------------ 


   let mut estado_ema=false;

    if !ema_no_espacios(valido_ema.clone()) && !ema_no_espacios_inicio_final(valido_ema.clone()) {
        valido_obs.push_str("EMAIL: No admite espacios en blanco. \n");
    }else if !ema_validar_punto(valido_ema.clone()){
        valido_obs.push_str("EMAIL: No debe tener punto antes o despues de @. \n");
    }else if !ema_dominio(valido_ema.clone()){
        valido_obs.push_str("EMAIL: El dominio debe tener entre 2 y 6 letras. \n");
    }else if !ema_caracteres(valido_ema.clone()){
        valido_obs.push_str("EMAIL: Antes del @ solo se acepta numeros, letras, punto, guión, guión bajo. \n");
    }else {
        estado_ema=true
    }


    if valido_ema==""{ estado_ema=false}
    if valido_fec==""{ estado_fec=false}
    if valido_gen==""{ estado_gen=false}
    if valido_id ==""{ estado_id=false}
    if valido_nom==""{ estado_nom=false}
    if valido_tel==""{ estado_tel=false}
    if valido_est==""{ estado_est=false}
    if valido_dir==""{ estado_dir=false}

    

//------------------------VALIDAR-------------------------------
    if estado_nom && estado_tel && estado_ema {
        valido_val=true;
    }
    else{ valido_val=false;}


//-------------------GUARDAR DATOS EN A ESTRUCTURA---------------
  
let per1 = models::StructPersona {
    identificacion: valido_id,
    nombre: valido_nom,
    genero: valido_gen,
    estadocivil: valido_est,
    fechanacimiento: valido_fec, 
    telefono: valido_tel,
    direccion: valido_dir,
    email: valido_ema,
    validado: valido_val,
    observacion: valido_obs,
};
return per1;




}