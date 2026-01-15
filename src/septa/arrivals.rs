use crate::URL;
use crate::traits::{Parse, PrettyPrint};
use crate::utils::parse_datetime;
use anyhow::{Context, Result};
use colored::Colorize;
use serde::Deserialize;
use std::collections::HashMap;
use url::Url;

#[derive(Deserialize, Clone)]
pub struct Train {
    train_id: Option<String>,
    destination: Option<String>,
    status: Option<String>,
    next_station: Option<String>,
    sched_time: Option<String>,
}

/// Intermediate structure matching the SEPTA API response format.
/// The API returns: `{ "station_name": [{"Northbound": [...]}, {"Southbound": [...]}] }`
#[derive(Deserialize)]
struct ApiResponse(HashMap<String, Vec<HashMap<String, Vec<Train>>>>);

/// Cleaned up arrivals data with northbound and southbound trains.
pub struct Arrivals {
    pub northbound: Vec<Train>,
    pub southbound: Vec<Train>,
}

impl Arrivals {
    pub fn get(name: &str, num: u8) -> Result<Arrivals> {
        let request_url = Url::parse(&format!("{}/Arrivals/index.php?station={}&results={}", URL, name, num))?;

        let response: ApiResponse = ureq::get(request_url.as_ref()).call()?.body_mut().read_json()?;

        Self::from_response(response)
    }

    fn from_response(response: ApiResponse) -> Result<Arrivals> {
        let directions = response.0.into_values().next().context("Empty response from API")?;

        let mut northbound = Vec::new();
        let mut southbound = Vec::new();

        for direction_map in directions {
            if let Some(trains) = direction_map.get("Northbound") {
                northbound.extend(trains.iter().cloned());
            }
            if let Some(trains) = direction_map.get("Southbound") {
                southbound.extend(trains.iter().cloned());
            }
        }

        Ok(Arrivals { northbound, southbound })
    }
}

impl Parse for Arrivals {
    fn parse(&self) -> Vec<String> {
        let format_train = |direction: &str, train: &Train| {
            format!(
                "{:<13}{:<11}{:<27}{:<12}{:<10}{}",
                direction,
                train.train_id.as_deref().unwrap_or("None"),
                train.next_station.as_deref().unwrap_or("None"),
                parse_datetime(train.sched_time.as_deref()),
                train.status.as_deref().unwrap_or("None"),
                train.destination.as_deref().unwrap_or("None")
            )
        };

        let north = self.northbound.iter().map(|t| format_train("North", t));
        let south = self.southbound.iter().map(|t| format_train("South", t));

        north.chain(south).collect()
    }
}

impl PrettyPrint for Arrivals {
    fn print(&self) {
        println!(
            "{:<13}{:<11}{:<27}{:<12}{:<10}{}",
            "Direction".blue(),
            "Train #".cyan(),
            "Next Station".green(),
            "Time".magenta(),
            "Status".red(),
            "Destination".yellow(),
        );
        for train in self.parse() {
            println!("{train}");
        }
    }
}
