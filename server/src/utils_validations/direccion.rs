



 //======================================      DIRECCION       =======================================
//======================================================================================================


// Para ser considerada una dirección válida debe contener
// mínimo 2 palabras separadas por espacio. Se considera
// dirección o sector como dato válido.
 pub fn dir_minimo_de_palabras(c:String)->bool{
    let arr=c.split_whitespace();
    let b=arr.count();
    if b>=2{
       return true;
    }
    return false;
}
