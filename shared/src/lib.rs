use serde::Serialize;
use serde::Deserialize;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeData{

    pub hour: i64,
    pub minute: i64,
    pub seconds: i64,
}

impl TimeData{

    pub fn display(&self) -> String{

        //{:02} prints the numbers with 2 leading zeroes
        format!( "{}:{:02}:{:02}", self.hour, self.minute, self.seconds  )
    }
}