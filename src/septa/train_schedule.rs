use crate::URL;
use crate::traits::{Parse, PrettyPrint};
use anyhow::Result;
use colored::Colorize;
use serde::Deserialize;
use url::Url;

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
        let request_url = Url::parse(&format!("{}/RRSchedules/index.php?req1={}", URL, num))?;
        let result: TrainSchedule = ureq::get(request_url.as_ref()).call()?.body_mut().read_json()?;
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
                    train.station.as_deref().unwrap_or("None"),
                    train.sched_tm.as_deref().unwrap_or("None"),
                    train.act_tm.as_deref().unwrap_or("None")
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
