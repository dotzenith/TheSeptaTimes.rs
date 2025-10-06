use crate::traits::{ParseWithMode, PrettyPrintWithMode};
use anyhow::Result as AnyResult;
use colored::Colorize;
use serde::Deserialize;
use std::env;
use std::str::FromStr;
use url::Url;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScheduleInner {
    departure_time: String,
    arrival_time: String,
    train_id: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScheduleOuter {
    weekday: Vec<ScheduleInner>,
    weekend: Vec<ScheduleInner>,
}

#[derive(Clone, Debug)]
pub enum ScheduleMode {
    Weekday,
    Weekend,
}

impl FromStr for ScheduleMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "weekday" => Ok(ScheduleMode::Weekday),
            "weekend" => Ok(ScheduleMode::Weekend),
            _ => Err(format!("Invalid mode: {}", s)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ScheduleDirection {
    Inbound,
    Outbound,
}

impl FromStr for ScheduleDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "inbound" => Ok(ScheduleDirection::Inbound),
            "outbound" => Ok(ScheduleDirection::Outbound),
            _ => Err(format!("Invalid direction: {}", s)),
        }
    }
}

impl ToString for ScheduleDirection {
    fn to_string(&self) -> String {
        match self {
            ScheduleDirection::Inbound => "inbound".to_string(),
            ScheduleDirection::Outbound => "outbound".to_string(),
        }
    }
}

impl ScheduleOuter {
    pub fn get(line: &str, direction: &ScheduleDirection, orig: &str, dest: &str) -> AnyResult<ScheduleOuter> {
        let base_url = match env::var("SeptumURL") {
            Ok(url) => url,
            Err(_) => {
                eprintln!("SeptumURL unset, cannot use this endpoint otherwise");
                std::process::exit(1)
            }
        };
        let request_url = Url::parse(&format!(
            "{}/schedule?line={}&direction={}&orig={}&dest={}",
            base_url,
            line,
            direction.to_string(),
            orig,
            dest
        ))?;
        let result: ScheduleOuter = ureq::get(request_url.as_ref()).call()?.body_mut().read_json()?;
        Ok(result)
    }
}

impl ParseWithMode for ScheduleOuter {
    fn parse(&self, mode: &ScheduleMode) -> Vec<String> {
        match mode {
            ScheduleMode::Weekday => self
                .weekday
                .iter()
                .map(|train| {
                    format!(
                        "{:<8}{:<14}{}",
                        train.train_id.as_str(),
                        parse_time(train.departure_time.as_str()),
                        parse_time(train.arrival_time.as_str()),
                    )
                })
                .collect(),
            ScheduleMode::Weekend => self
                .weekend
                .iter()
                .map(|train| {
                    format!(
                        "{:<8}{:<14}{}",
                        train.train_id.as_str(),
                        parse_time(train.departure_time.as_str()),
                        parse_time(train.arrival_time.as_str()),
                    )
                })
                .collect(),
        }
    }
}

impl PrettyPrintWithMode for ScheduleOuter {
    fn print(&self, mode: &ScheduleMode) {
        println!(
            "{:<8}{:<14}{}",
            "Train".yellow(),
            "Depart Time".cyan(),
            "Arrive Time".green(),
        );
        for train in self.parse(mode).iter() {
            println!("{train}");
        }
    }
}

fn parse_time(time: &str) -> String {
    let time_vec: Vec<&str> = time.split(":").collect();
    let mut hour = time_vec[0].parse::<u8>().unwrap_or(0);
    let minute = time_vec[1].parse::<u8>().unwrap_or(0);
    let mut meridian = "AM";

    // Look, this time handling was super crude to begin with
    // But septa also believes there are more than 24 hours in
    // a day, so here we are
    match hour {
        12 => {
            meridian = "PM";
        }
        13..=23 => {
            meridian = "PM";
            hour -= 12;
        }
        24 => {
            meridian = "AM";
            hour -= 12;
        }
        25..36 => {
            meridian = "AM";
            hour -= 24;
        }
        _ => (),
    }
    format!("{:02}:{:02} {}", hour, minute, meridian)
}
