use crate::data::{Arrival, NextToArrive, TrainSchedule, URL};
use reqwest::blocking::get;
use std::error::Error;

impl Arrival {
    pub fn get(name: &str, num: u8) -> Result<Arrival, Box<dyn Error>> {
        let request_url = format!(
            "{}/Arrivals/index.php?station={}&results={}",
            URL,
            name,
            num,
        );
        let result = Arrival(get(request_url)?.json()?);
        Ok(result)
    }
}

impl NextToArrive {
    pub fn get(from: &str, to: &str, num: u8) -> Result<NextToArrive, Box<dyn Error>> {
        let request_url = format!(
            "{}/NextToArrive/index.php?req1={}&req2={}&req3={}",
            URL,
            from,
            to,
            num,
        );
        let result = NextToArrive(get(request_url)?.json()?);
        Ok(result)
    }
}

impl TrainSchedule {
    pub fn get(num: &str) -> Result<TrainSchedule, Box<dyn Error>> {
        let request_url = format!("{}/RRSchedules/index.php?req1={}", URL, num);
        let result = TrainSchedule(get(request_url)?.json()?);
        Ok(result)
    }
}
