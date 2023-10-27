use crate::parse::{ArrivalResult, NextToArriveResult, TrainScheduleResult};
use reqwest::blocking::get;
use simsearch::SimSearch;
use std::error::Error;

#[derive(Debug)]
pub struct SeptaApi {
    base_url: &'static str,
    stations: [&'static str; 154],
    search_engine: SimSearch<usize>,
}

impl SeptaApi {
    pub fn next_to_arrive(
        &self,
        from: &str,
        to: &str,
        num: u8,
    ) -> Result<NextToArriveResult, Box<dyn Error>> {
        let request_url = format!(
            "{}/NextToArrive/index.php?req1={}&req2={}&req3={}",
            self.base_url,
            from,
            to,
            num,
        );
        let result = NextToArriveResult(get(request_url)?.json()?);
        Ok(result)
    }

    pub fn arrivals(&self, name: &str, num: u8) -> Result<ArrivalResult, Box<dyn Error>> {
        let request_url = format!(
            "{}/Arrivals/index.php?station={}&results={}",
            self.base_url,
            name,
            num,
        );
        let result = ArrivalResult(get(request_url)?.json()?);
        Ok(result)
    }

    pub fn train_schedule(&self, num: &str) -> Result<TrainScheduleResult, Box<dyn Error>> {
        let request_url = format!("{}/RRSchedules/index.php?req1={}", self.base_url, num);
        let result = TrainScheduleResult(get(request_url)?.json()?);
        Ok(result)
    }

    pub fn get_station(&self, station: &str) -> Result<&'static str, Box<dyn Error>> {
        let results: Vec<usize> = self.search_engine.search(station);
        let best_match = results.get(0).ok_or("No Matches")?.to_owned();
        Ok(self.stations[best_match])
    }

    fn populate_search_engine(&mut self) -> () {
        for (id, entry) in self.stations.iter().enumerate() {
            self.search_engine.insert(id, entry);
        }
    }

    pub fn new() -> Self {
        let mut api = SeptaApi {
            base_url: "https://www3.septa.org/api",
            search_engine: SimSearch::new(),
            stations: crate::stations::STATIONS,
        };
        api.populate_search_engine();
        api
    }
}
