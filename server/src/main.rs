use actix_web::http::header::HttpDate;
use actix_web::{App, HttpServer, HttpRequest};
mod generated;
use actix_web::{HttpResponse};
use actix_web::{web, get, post};
use serde_json::Value;
use shared::TimeData;


//returns (Hour, Minute, Seconds)
async fn fetch_amsterdam_time( )  -> Option<TimeData>{

    let url = "https://www.timeapi.io/api/Time/current/zone?timeZone=Europe/Amsterdam";

    let client = reqwest::blocking::Client::new();

    //Actix web 3.0 uses an old version of the tokio runtime
    //you can either find an old enough version of reqwest that matches
    //Or you can just do this and use the blocking client

    let result = client.get(url)
        //this sets the headers to request json
        .header( "Content-Type", "application/json" )
        //you can alternatively use .json( &X )
        //where X is some json serializable data struct to send as the body
        //and it also sets the Content-Type to application/json
        .send();


    if let Ok(response) = result{
        
        if let Ok(value) = response.json::<serde_json::Value>(){

            let hour = value.get("hour")?.as_i64()?;
            let minute = value.get("minute")?.as_i64()?;
            let seconds = value.get("seconds")?.as_i64()?;

            let timedata = shared::TimeData{hour, minute, seconds};


            return Some( timedata );
        }
    }

    return None;
}


//get the time, a serialized 
#[actix_web::get("/api/get_time/")]
async fn get_time( )-> HttpResponse {

    println!("Someone is requesting the time in Amsterdam");

    if let Some(time) = fetch_amsterdam_time().await{

        return HttpResponse::Accepted().json( time );
    }

    return HttpResponse::InternalServerError().body("");
}



#[actix_web::main]
async fn main()  {

    println!("application starting");

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
