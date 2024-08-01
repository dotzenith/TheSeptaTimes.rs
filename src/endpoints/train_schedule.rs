use crate::traits::{Parse, PrettyPrint};
use crate::URL;
use anyhow::Result;
use colored::Colorize;
use reqwest::blocking::get;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct TrainScheduleInner {
    station: Option<String>,
    sched_tm: Option<String>,
    est_tm: Option<String>,
    act_tm: Option<String>,
}

#[derive(Deserialize)]
pub struct TrainSchedule(pub Vec<TrainScheduleInner>);

impl TrainSchedule {
    pub fn get(num: &str) -> Result<TrainSchedule> {
        let request_url = format!("{}/RRSchedules/index.php?req1={}", URL, num);
        let result: TrainSchedule = get(request_url)?.json()?;
        Ok(result)
    }
}

impl Parse for TrainSchedule {
    fn parse(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|train| {
                format!(
                    "{:<27}{:<18}{}",
                    train.station.as_ref().map_or("None", |orig| orig.as_str()),
                    train.sched_tm.as_ref().map_or("None", |orig| orig.as_str()),
                    train.act_tm.as_ref().map_or("None", |orig| orig.as_str())
                )
            })
            .collect()
    }
}

impl PrettyPrint for TrainSchedule {
    fn print(&self) {
        println!(
            "{:<27}{:<18}{}",
            "Station".yellow(),
            "Scheduled Time".cyan(),
            "Actual Time".green(),
        );
        for train in self.parse().iter() {
            println!("{train}");
        }
    }
}
