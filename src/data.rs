use anyhow::{anyhow, Context, Result};
use bincode::{deserialize_from, serialize_into};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use platform_dirs::AppDirs;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::fs::{create_dir, OpenOptions};
use std::io::{BufReader, BufWriter};

pub struct NextToArrive(pub Vec<HashMap<String, Option<String>>>);
pub struct Arrival(pub HashMap<String, Vec<HashMap<String, Vec<HashMap<String, Option<String>>>>>>);
pub struct TrainSchedule(pub Vec<HashMap<String, Option<String>>>);

pub struct Stations {
    stations: Vec<String>,
    matcher: SkimMatcherV2,
}

impl Stations {
    pub fn new() -> Self {
        let mut stations = Stations {
            stations: Vec::new(),
            matcher: SkimMatcherV2::default(),
        };
        stations.stations = match Self::get_stations_from_file_or_url() {
            Ok(stations) => stations,
            Err(_) => FALLBACK_STATIONS.into_iter().map(|str| str.to_string()).collect(),
        };
        stations
    }

    pub fn get_stations(&self) -> &Vec<String> {
        &self.stations
    }

    pub fn refresh() -> Result<()> {
        let station = Self::fetch_stations_from_url("http://www3.septa.org/VIRegionalRail.html")?;
        Self::save_stations_to_file(&station)?;
        Ok(())
    }

    pub fn fuzzy_search(&self, search: &str) -> Result<&str> {
        let results: Vec<i64> = self
            .stations
            .iter()
            .map(|station| self.matcher.fuzzy_match(station, search).unwrap_or(0))
            .collect();

        if results.iter().sum::<i64>() == 0 {
            return Err(anyhow!("No matching value"));
        }

        Ok(&self.stations[results
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .context("Could not get the index of matching value")?])
    }

    fn get_stations_from_file_or_url() -> Result<Vec<String>> {
        match Self::read_stations_from_file() {
            Ok(stations) => Ok(stations),
            Err(_) => {
                let station = Self::fetch_stations_from_url("http://www3.septa.org/VIRegionalRail.html")?;
                Self::save_stations_to_file(&station)?;
                Ok(station)
            }
        }
    }

    fn fetch_stations_from_url(url: &str) -> Result<Vec<String>> {
        let response = reqwest::blocking::get(url);
        let html_content = Html::parse_document(&response?.text()?);
        let selector = Selector::parse("table > tbody > tr > td").unwrap();

        let stations: Vec<String> = html_content
            .select(&selector)
            .skip(1)
            .step_by(2)
            .map(|content| content.inner_html().to_owned())
            .collect();
        Ok(stations)
    }

    fn save_stations_to_file(stations: &Vec<String>) -> Result<()> {
        let app_dirs = AppDirs::new(Some("TheSeptaTimes"), true).context("Unable to get AppDirs")?;
        if !app_dirs.cache_dir.exists() {
            create_dir(&app_dirs.cache_dir)?;
        }

        let mut file = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(app_dirs.cache_dir.join("stations"))?,
        );

        serialize_into(&mut file, stations)?;
        Ok(())
    }

    fn read_stations_from_file() -> Result<Vec<String>> {
        let app_dirs = AppDirs::new(Some("TheSeptaTimes"), true).context("Unable to get AppDirs")?;

        let mut f = BufReader::new(
            OpenOptions::new()
                .read(true)
                .open(app_dirs.cache_dir.join("stations"))?,
        );

        let stations: Vec<String> = deserialize_from(&mut f)?;
        Ok(stations)
    }
}

pub const URL: &str = "https://www3.septa.org/api";

const FALLBACK_STATIONS: [&'static str; 154] = [
    "9th St",
    "30th Street Station",
    "49th St",
    "Airport Terminal A",
    "airport terminal B",
    "airport terminal C-D",
    "airport terminal E-F",
    "Allegheny",
    "Allen Lane",
    "Ambler",
    "Angora",
    "Ardmore",
    "Ardsley",
    "Aala",
    "Aerwyn",
    "Aethayres",
    "Aridesburg",
    "Aristol",
    "Bryn Mawr",
    "Carpenter",
    "Chalfont",
    "Chelten Avenue",
    "Cheltenham",
    "Chester TC",
    "Chestnut Hill East",
    "Chestnut Hill West",
    "Churchmans Crossing",
    "Claymont",
    "Clifton-Aldan",
    "Colmar",
    "Conshohocken",
    "Cornwells Heights",
    "Crestmont",
    "Croydon",
    "Crum Lynne",
    "Curtis Park",
    "Cynwyd",
    "Darby",
    "Daylesford",
    "Delaware Valley College",
    "Devon",
    "Downingtown",
    "Doylestown",
    "East Falls",
    "Eastwick Station",
    "Eddington",
    "Eddystone",
    "Elkins Park",
    "Elm St",
    "Elwyn Station",
    "Exton",
    "Fern Rock TC",
    "Fernwood",
    "Folcroft",
    "Forest Hills",
    "Ft Washington",
    "Fortuna",
    "Fox Chase",
    "Germantown",
    "Gladstone",
    "Glenside",
    "Gravers",
    "Gwynedd Valley",
    "Hatboro",
    "Haverford",
    "Highland",
    "Highland Ave",
    "Holmesburg Jct",
    "Ivy Ridge",
    "Market East",
    "Jenkintown-Wyncote",
    "Langhorne",
    "Lansdale",
    "Lansdowne",
    "Lawndale",
    "Levittown",
    "Link Belt",
    "Main St",
    "Malvern",
    "Manayunk",
    "Marcus Hook",
    "Meadowbrook",
    "Media",
    "Melrose Park",
    "Merion",
    "Miquon",
    "Morton",
    "Mt Airy",
    "Moylan-Rose Valley",
    "Narberth",
    "Neshaminy Falls",
    "New Britain",
    "Newark",
    "Noble",
    "Norristown TC",
    "North Broad St",
    "North Hills",
    "North Philadelphia",
    "North Wales",
    "Norwood",
    "Olney",
    "Oreland",
    "Overbrook",
    "Paoli",
    "Penllyn",
    "Pennbrook",
    "Penn Medicine Station",
    "Philmont",
    "Primos",
    "Prospect Park",
    "Queen Lane",
    "Radnor",
    "Ridley Park",
    "Rosemont",
    "Roslyn",
    "Rydal",
    "Ryers",
    "Secane",
    "Sedgwick",
    "Sharon Hill",
    "Somerton",
    "Spring Mill",
    "St. Davids",
    "St. Martins",
    "Stenton",
    "Strafford",
    "Suburban Station",
    "Swarthmore",
    "Swarthmore",
    "Tacony",
    "Temple U",
    "Thorndale",
    "Torresdale",
    "Trenton",
    "Trevose",
    "Tulpehocken",
    "Upsal",
    "Villanova",
    "Wallingford",
    "Warminster",
    "Washington Lane",
    "Wayne",
    "Wayne Jct",
    "West Trenton",
    "Whitford",
    "Willow Grove",
    "Wilmington",
    "Wissahickon",
    "Wister",
    "Woodbourne",
    "Wyndmoor",
    "Wynnefield Avenue",
    "Wynnewood",
    "Yardley",
];
