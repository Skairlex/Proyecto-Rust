
use actix_web::{web, App, HttpServer};
use std::io::Write;
use actix_multipart::Multipart;
use actix_web::{middleware, Error, HttpResponse};
use futures::{StreamExt, TryStreamExt};
 

//API MULTIPART
#[actix_web::main]
pub async fn turn_on() -> std::io::Result<()> {
    let ip = "localhost:3000";
    HttpServer::new(|| {
        App::new()
            .route("/str", web::get().to(|| async { "Hola Rust" }))
            
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/")
                    .route(web::get().to(its_ready))
                    .route(web::post().to(save_file)),
                   
                    
            )
    })
    .bind(ip)?
    .run()
    .await
}

/*
fn create_dir_temp(){
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::fs::create_dir_all("./tmp").unwrap_or_else(|e| panic!("Error creating dir: {}", e));

}
*/

 async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // create_dir_temp();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}


//TO DO 
pub fn its_ready() -> HttpResponse {
   HttpResponse::Ok().into()
}
