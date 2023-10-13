use std::collections::HashMap;

pub type NextToArriveResult = Vec<HashMap<String, Option<String>>>;
pub type ArrivalResult = HashMap<String, Vec<HashMap<String, Vec<HashMap<String, Option<String>>>>>>;
pub type TrainScheduleResult = Vec<HashMap<String, Option<String>>>;

pub trait Parse {
    fn parse(&self) -> Vec<String>;
}

impl Parse for NextToArriveResult {
    fn parse(&self) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        for train in self.iter() {
            results.push(format!(
                "{:<11}{:<13}{:<11}{:<9}{}",
                train["orig_train"].as_ref().unwrap(),
                train["orig_departure_time"].as_ref().unwrap(),
                train["orig_arrival_time"].as_ref().unwrap(),
                train["orig_delay"].as_ref().unwrap(),
                train["orig_line"].as_ref().unwrap()
            ));
        }
        results
    }
}
