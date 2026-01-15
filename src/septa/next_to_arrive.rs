use crate::URL;
use crate::traits::{Parse, PrettyPrint};
use anyhow::Result;
use colored::Colorize;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
pub struct NextToArriveInner {
    orig_train: Option<String>,
    orig_line: Option<String>,
    orig_departure_time: Option<String>,
    orig_arrival_time: Option<String>,
    orig_delay: Option<String>,
    term_train: Option<String>,
    term_line: Option<String>,
    term_depart_time: Option<String>,
    term_arrival_time: Option<String>,
    connection: Option<String>,
    term_delay: Option<String>,
    isdirect: Option<String>,
}

#[derive(Deserialize)]
pub struct NextToArrive(pub Vec<NextToArriveInner>);

impl NextToArrive {
    pub fn get(from: &str, to: &str, num: u8) -> Result<NextToArrive> {
        let request_url = Url::parse(&format!(
            "{}/NextToArrive/index.php?req1={}&req2={}&req3={}",
            URL, from, to, num,
        ))?;
        let result: NextToArrive = ureq::get(request_url.as_ref()).call()?.body_mut().read_json()?;
        Ok(result)
    }
}

impl Parse for NextToArrive {
    fn parse(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|train| {
                if train.isdirect.as_deref().unwrap_or("false") == "true" {
                    format!(
                        "{:<11}{:<13}{:<11}{:<9}{}",
                        train.orig_train.as_deref().unwrap_or("None"),
                        train.orig_departure_time.as_deref().unwrap_or("None"),
                        train.orig_arrival_time.as_deref().unwrap_or("None"),
                        train.orig_delay.as_deref().unwrap_or("None"),
                        train.orig_line.as_deref().unwrap_or("None")
                    )
                } else {
                    let first = format!(
                        "{:<11}{:<13}{:<11}{:<9}{}",
                        train.orig_train.as_deref().unwrap_or("None"),
                        train.orig_departure_time.as_deref().unwrap_or("None"),
                        train.orig_arrival_time.as_deref().unwrap_or("None"),
                        train.orig_delay.as_deref().unwrap_or("None"),
                        train.orig_line.as_deref().unwrap_or("None")
                    );

                    let connection = format!("Connection: {}", train.connection.as_deref().unwrap_or("None")).blue();

                    let second = format!(
                        "{:<11}{:<13}{:<11}{:<9}{}",
                        train.term_train.as_deref().unwrap_or("None"),
                        train.term_depart_time.as_deref().unwrap_or("None"),
                        train.term_arrival_time.as_deref().unwrap_or("None"),
                        train.term_delay.as_deref().unwrap_or("None"),
                        train.term_line.as_deref().unwrap_or("None")
                    );
                    format!("{}\n{:^width$}\n{}\n", first, connection, second, width = first.len())
                }
            })
            .collect()
    }
}

impl PrettyPrint for NextToArrive {
    fn print(&self) {
        println!(
            "{:<11}{:<13}{:<11}{:<9}{}",
            "Train #".cyan(),
            "Departure".green(),
            "Arrival".magenta(),
            "Delay".red(),
            "Line".yellow(),
        );
        for train in self.parse().iter() {
            println!("{train}");
        }
    }
}
