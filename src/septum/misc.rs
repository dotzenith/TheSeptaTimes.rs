use super::ScheduleDirection;
use crate::traits::{Parse, PrettyPrint};
use anyhow::{Context, Result, anyhow};
use colored::Colorize;
use nucleo_matcher::{Config, Matcher, pattern};
use serde::Deserialize;
use std::env;
use url::Url;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct LinesInner {
    line_code: String,
    line_name: String,
}

#[derive(Deserialize, Debug)]
pub struct Lines(pub Vec<LinesInner>);

impl Parse for Lines {
    fn parse(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|item| format!("{:<6}{}", item.line_code, item.line_name))
            .collect()
    }
}

impl PrettyPrint for Lines {
    fn print(&self) {
        println!("{:<6}{}", "Code".blue(), "Name".green());
        for train in self.parse().iter() {
            println!("{train}");
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct LinesStationsInner {
    stop_id: String,
    stop_name: String,
}

#[derive(Deserialize, Debug)]
pub struct LineStations(pub Vec<LinesStationsInner>);

pub struct SeptumMisc {
    url: String,
    matcher: Matcher,
}

impl SeptumMisc {
    pub fn new() -> Result<Self> {
        let base_url = env::var("SeptumURL").context("SeptumURL not set, cannot use these endpoints otherwise")?;

        Ok(SeptumMisc {
            url: base_url,
            matcher: Matcher::new(Config::DEFAULT),
        })
    }

    pub fn get_lines(&self) -> Result<Lines> {
        let request_url = format!("{}/schedule/lines", &self.url);
        let result: Lines = ureq::get(request_url).call()?.body_mut().read_json()?;
        Ok(result)
    }

    pub fn get_stations_for_line(&self, line: &str, direction: &ScheduleDirection) -> Result<Vec<String>> {
        let request_url = Url::parse(&format!(
            "{}/schedule/stations?line={}&direction={}",
            self.url,
            line,
            direction.to_string()
        ))?;
        let result: LineStations = ureq::get(request_url.as_ref()).call()?.body_mut().read_json()?;
        let stations: Vec<String> = result.0.into_iter().map(|item| item.stop_name).collect();
        Ok(stations)
    }
    pub fn fuzzy_match_station_for_line(
        &mut self,
        line: &str,
        search: &str,
        direction: &ScheduleDirection,
    ) -> Result<String> {
        let request_url = Url::parse(&format!(
            "{}/schedule/stations?line={}&direction={}",
            self.url,
            line,
            direction.to_string()
        ))?;
        let result: LineStations = ureq::get(request_url.as_ref()).call()?.body_mut().read_json()?;
        let stations: Vec<String> = result.0.into_iter().map(|item| item.stop_name).collect();

        let matches = pattern::Pattern::new(
            search,
            pattern::CaseMatching::Ignore,
            pattern::Normalization::Smart,
            pattern::AtomKind::Fuzzy,
        )
        .match_list(&stations, &mut self.matcher);

        if matches.len() == 0 {
            return Err(anyhow!("No matching value"));
        }

        let station = matches[0].0.to_owned();

        Ok(station)
    }
}
