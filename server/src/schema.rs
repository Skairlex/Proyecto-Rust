table! {
    persona (codigo) {
        codigo -> Int4,
        identificacion -> Nullable<Varchar>,
        nombre -> Nullable<Text>,
        genero -> Nullable<Varchar>,
        estadocivil -> Nullable<Varchar>,
        fechanacimiento -> Nullable<Text>,
        telefono -> Nullable<Varchar>,
        direccion -> Nullable<Text>,
        email -> Nullable<Varchar>,
        validado -> Nullable<Bool>,
        observacion -> Nullable<Text>,
    }
}
