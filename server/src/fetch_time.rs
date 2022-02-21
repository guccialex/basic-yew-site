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
