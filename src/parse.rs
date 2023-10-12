use std::collections::HashMap;

pub type NextToArriveResult = Vec<HashMap<String, Option<String>>>;
pub type ArrivalResult = HashMap<String, Vec<HashMap<String, Vec<HashMap<String, Option<String>>>>>>;
pub type TrainScheduleResult = Vec<HashMap<String, Option<String>>>;

pub trait Parse {
    fn parse(&self) -> Vec<String>;
}

impl Parse for NextToArriveResult {
    fn parse(&self) -> Vec<String> {
        println!("{:?}", self);
        vec!["Hmmmm".to_owned()]
    }
}
