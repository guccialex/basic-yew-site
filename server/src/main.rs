use actix_web::{App, HttpServer};
mod generated;
use actix_web::{HttpResponse };
use actix_web::{web};
use actix_web::{get};








#[actix_web::main]
async fn main()  {

    println!("Starting application");


    HttpServer::new(move || {

        let generated = generated::generate();

        App::new()     
            .service(actix_web_static_files::ResourceFiles::new("/", generated).resolve_not_found_to_root() )

    })
    .bind(("0.0.0.0", 8080)).unwrap()
    .run()
    .await.unwrap();



}
