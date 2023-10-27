use std::collections::HashMap;

pub struct NextToArriveResult(pub Vec<HashMap<String, Option<String>>>);
pub struct ArrivalResult(
    pub HashMap<String, Vec<HashMap<String, Vec<HashMap<String, Option<String>>>>>>,
);
pub struct TrainScheduleResult(pub Vec<HashMap<String, Option<String>>>);

pub fn parse_time(time: Option<&str>) -> String {
    if time.is_none() {
        return "None".to_owned();
    }

    let time_vec: Vec<&str> = time.unwrap().split(" ").collect::<Vec<&str>>()[1]
        .split(":")
        .collect();
    let mut hour = time_vec[0].parse::<u8>().unwrap_or(0);
    let minute = time_vec[1].parse::<u8>().unwrap_or(0);
    let mut meridian = "AM";

    if hour > 12 {
        hour -= 12;
        meridian = "PM";
    }
    format!("{:02}:{:02} {}", hour, minute, meridian)
}

pub trait Parse {
    fn parse(&self) -> Vec<String>;
}

impl Parse for NextToArriveResult {
    fn parse(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|train| {
                format!(
                    "{:<11}{:<13}{:<11}{:<9}{}",
                    train["orig_train"].as_deref().unwrap_or("balls"),
                    train["orig_departure_time"].as_deref().unwrap_or("balls"),
                    train["orig_arrival_time"].as_deref().unwrap_or("balls"),
                    train["orig_delay"].as_deref().unwrap_or("balls"),
                    train["orig_line"].as_deref().unwrap_or("balls")
                )
            })
            .collect()
    }
}

impl Parse for ArrivalResult {
    fn parse(&self) -> Vec<String> {
        let vec = self.0.values().next().unwrap();
        let north = &vec[0]["Northbound"];
        let south = &vec[1]["Southbound"];

        let mut trains: Vec<String> = north
            .into_iter()
            .map(|train| {
                format!(
                    "{:<13}{:<11}{:<27}{:<12}{:<10}{}",
                    "North",
                    train["train_id"].as_deref().unwrap_or("balls"),
                    train["next_station"].as_deref().unwrap_or("balls"),
                    parse_time(train["sched_time"].as_deref()),
                    train["status"].as_deref().unwrap_or("balls"),
                    train["destination"].as_deref().unwrap_or("balls")
                )
            })
            .collect();

        trains.append(
            &mut south.into_iter()
            .map(|train| {
                format!(
                    "{:<13}{:<11}{:<27}{:<12}{:<10}{}",
                    "South",
                    train["train_id"].as_deref().unwrap_or("balls"),
                    train["next_station"].as_deref().unwrap_or("balls"),
                    parse_time(train["sched_time"].as_deref()),
                    train["status"].as_deref().unwrap_or("balls"),
                    train["destination"].as_deref().unwrap_or("balls")
                )
            })
            .collect::<Vec<String>>()
        );
        trains
    }
}

impl Parse for TrainScheduleResult {
    fn parse(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|train| {
                format!(
                    "{:<27}{:<18}{}",
                    train["station"].as_deref().unwrap_or("balls"),
                    train["sched_tm"].as_deref().unwrap_or("balls"),
                    train["act_tm"].as_deref().unwrap_or("balls")
                )
            })
            .collect()
    }
}
