use yew::prelude::*;


enum Msg {
    AddOne,

    FetchTime, 

    GotTime( String ),

    Error,
}

struct Model {
    value: i64,

    timestring: Option<String>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            timestring: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
            },

            Msg::FetchTime => {
                
                ctx.link().send_future(
                    async move {
        
                        let baseurl = web_sys::window().unwrap().origin();
        
                        let url = baseurl + "/api/get_time/";
                    
                        let client = reqwest::Client::new();
                    
                        let future = client.get(url)
                            .send();
        
                        if let Ok(response) = future.await{
                            if let Ok( timestring) = response.json::<String>().await{

                                return Msg::GotTime( timestring );
                            }
                        }
        
                        return Msg::Error;
                    }
                );
            },
            Msg::GotTime( timestring ) =>{

                self.timestring = Some(timestring);
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

        if let Some(temp) = self.timestring.clone(){

            timestring = html!{
                {temp}
            };
        }
        else{
            timestring = html!{};
        }


        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>

                <button onclick={link.callback(|_| Msg::FetchTime)}> {"Fetch Time From Server"} </button>

                {timestring}
                
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}