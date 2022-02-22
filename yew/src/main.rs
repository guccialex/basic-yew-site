use yew::prelude::*;



//returns (Hour, Minute, Seconds)
async fn fetch_amsterdam_time( )  -> Option<(i64,i64,i64)>{

    let url = "http://worldtimeapi.org/api/timezone";

    let client = reqwest::Client::new();

    //Actix web 3.0 uses an old version of the tokio runtime
    //you can either find an old enough version of reqwest that matches
    //but I just use the blocking client

    let result = client.get(url)
        //this sets the headers to request json
        .header( "Content-Type", "application/json" )
        //you can alternatively use .json( &X )
        //where X is some json serializable data struct to send as the body
        //and also sets the Conent-Type to application/json
        .send();


    if let Ok(response) = result.await{
        
        if let Ok(value) = response.json::<serde_json::Value>().await{

            let hour = value.get("hour")?.as_i64()?;
            let minute = value.get("minute")?.as_i64()?;
            let seconds = value.get("seconds")?.as_i64()?;

            return Some( (hour, minute, seconds) );
        }
    }

    return None;
}




enum Msg {
    FetchTime, 

    GotTime( (i64, i64, i64) ),

    Error,
}

struct Model {

    time: Option<(i64,i64,i64)>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            time: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {

            Msg::FetchTime => {
                

                gloo::console::log!("fetching");
                ctx.link().send_future(
                    async move {
        
                        let baseurl = web_sys::window().unwrap().origin();
        
                        let url = baseurl + "/api/get_time/";
                    
                        let client = reqwest::Client::new();
                    
                        let future = client.get(url)
                            .send();

                        gloo::console::log!("sendt");


                        let temp = future.await;
        
                        if let Ok(response) = temp{

                            gloo::console::log!("doff");


                            if let Ok( time) = response.json::<(i64,i64,i64)>().await{
                                gloo::console::log!("godt");

                                return Msg::GotTime( time );
                            }
                        }
                        else if let Err( x) =temp{

                            gloo::console::log!( &format!("{:?}",x) );
                        }
        
                        return Msg::Error;
                    }
                );
            },
            Msg::GotTime( time ) =>{

                self.time = Some(time);
            },
            Msg::Error =>{

            }


        };

        true

    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();


        let timestring;

        if let Some(temp) = self.time.clone(){

            timestring = html!{
                {temp.2}
            };
        }
        else{
            timestring = html!{};
        }


        html! {
            <div>

                <button onclick={link.callback(|_| Msg::FetchTime)}> {"Fetch Time From Server"} </button>

                {timestring}
                
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}    //whatever api you call must have the same origin or allow CORS