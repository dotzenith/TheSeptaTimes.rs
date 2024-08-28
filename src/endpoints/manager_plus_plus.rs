use anyhow::{Context, Result};
use reqwest::blocking::get;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct LinesInner {
    line_code: String,
    line_name: String,
}

#[derive(Deserialize, Debug)]
pub struct Lines(pub Vec<LinesInner>);

#[derive(Debug)]
pub struct SeptaPlusPlusManager {
    url: String,
}

impl SeptaPlusPlusManager {
    pub fn new() -> Result<Self> {
        let base_url =
            env::var("SeptaPlusPlusURL").context("SeptaPlusPlusURL not set, cannot use these endpoints otherwise")?;

        Ok(SeptaPlusPlusManager { url: base_url })
    }

    pub fn get_lines(&self) -> Result<Vec<String>> {
        let request_url = format!("{}/schedule/lines", &self.url);
        let result: Lines = get(request_url)?.json()?;
        Ok(result.0.into_iter().map(|item| item.line_code).collect())
    }
}
