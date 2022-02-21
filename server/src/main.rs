use actix_web::{App, HttpServer};
mod generated;
use actix_web::{HttpResponse };
use actix_web::{web};
use actix_web::{get};
use actix_web::{post};
use serde_json::Value;


//returns (Hour, Minute, Seconds)
async pub fn fetch_amsterdam_time( )  -> Option<(i64,i64,i64)>{

    //documentation of this api at https://www.timeapi.io/swagger/index.html
    //whatever api you call must have the same origin or allow CORS
    let url = "http://worldtimeapi.org/api/timezone";

    let client = reqwest::Client::new();

    let future = client.get(url)
        //this sets the headers to request json
        .header( "Content-Type", "application/json" )
        //you can alternatively use .json( &X )
        //where X is some json serializable data struct to send as the body
        //and also sets the Conent-Type to application/json
        .send();


    if let Ok(response) = future.await{
        
        if let Ok(value) = response.json::<serde_json::Value>().await{

            let hour = value.get("hour")?.as_i64()?;
            let minute = value.get("minute")?.as_i64()?;
            let seconds = value.get("seconds")?.as_i64()?;

            return Some( (hour, minute, seconds) );
        }
    }

    return None;
}


//given the string id of the post, get the content and the 
#[actix_web::get("/api/get_time/")]
async fn get_time(  )-> HttpResponse {

    println!("a user is requesting a list of time zones");
    
    let url = format!("http://worldtimeapi.org/api/timezone/America/Toronto");

    let client = reqwest::blocking::Client::new();

    let future = client.get(url)
        .header("Accept", "application/json")
        .send();

    let response = future.unwrap();

    let contents: Value = response.json().unwrap();

    let time = &contents["datetime"];

    let time = time.to_string();

    return HttpResponse::Accepted().json( time );
}








#[actix_web::main]
async fn main()  {

    println!("Starting application");


    HttpServer::new(move || {

        let generated = generated::generate();

        App::new()     

            .service(get_time ) 

            .service(actix_web_static_files::ResourceFiles::new("/", generated).resolve_not_found_to_root() )

    })
    .bind(("0.0.0.0", 8080)).unwrap()
    .run()
    .await.unwrap();



}
