use yew::prelude::*;

use shared::TimeData;

//returns (Hour, Minute, Seconds)
async fn fetch_amsterdam_time( )  -> Option<TimeData>{

    let baseurl = web_sys::window().unwrap().origin();
        
    let url = baseurl + "/api/get_time/";

    let client = reqwest::Client::new();

    let future = client.get(url)
        .send();

    let temp = future.await;

    if let Ok(response) = temp{

        if let Ok( time) = response.json::<TimeData>().await{
            return Some( time );
        }
    }

    return None;
}




enum Msg {
    FetchTime, 

    GotTime( TimeData ),

    Error,
}

struct Model {

    time: Option<TimeData>,
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
                
                gloo::console::log!("fetching the time from the server");

                ctx.link().send_future(
                    async move {
        
                        if let Some(time) = fetch_amsterdam_time().await{
                            return Msg::GotTime( time );
                        }
                        else{
                            return Msg::Error;
                        }
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

        if let Some(time) = self.time.clone(){

            timestring = html!{
                <>
                {"It's "}{time.display()}{" in Amsterdam right now"}
                </>
            };
        }
        else{
            timestring = html!{};
        }

        html! {
            <div >

                <img style="height: 400px;" src="/static/images/1.png" alt="hyperbole and a half"/>

                <br/>

                <button onclick={link.callback(|_| Msg::FetchTime)}> {"Get the 24hr time in Amsterdam"} </button>

                <br/>
                {timestring}
                
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}    