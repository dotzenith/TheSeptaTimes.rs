use anyhow::{anyhow, Context, Result};
use bincode::{deserialize_from, serialize_into};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use platform_dirs::AppDirs;
use serde::Deserialize;
use std::env;
use std::fs;
use std::fs::{create_dir, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::time::SystemTime;

pub struct StationsManager {
    stations: Vec<String>,
    matcher: SkimMatcherV2,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct StationsInner {
    station_name: String,
    parameter: String,
}

#[derive(Deserialize, Debug)]
pub struct Stations(pub Vec<StationsInner>);

impl StationsManager {
    pub fn new() -> Self {
        let mut manager = StationsManager {
            stations: Vec::new(),
            matcher: SkimMatcherV2::default(),
        };
        manager.stations = match Self::get_stations_from_file_or_api() {
            Ok(stations) => stations,
            Err(_) => FALLBACK_STATIONS.into_iter().map(|str| str.to_string()).collect(),
        };
        manager
    }

    pub fn get_stations(&self) -> &Vec<String> {
        &self.stations
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

        let mut station: &str = &self.stations[results
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .context("Could not get the index of matching value")?];

        match station.split_once('(') {
            Some((first, _)) => station = first.trim(),
            None => (),
        }

        Ok(station)
    }

    fn get_stations_from_file_or_api() -> Result<Vec<String>> {
        match Self::read_stations_from_file() {
            Ok(stations) => Ok(stations),
            Err(_) => {
                let station = Self::fetch_stations_from_api()?;
                Self::save_stations_to_file(&station)?;
                Ok(station)
            }
        }
    }

    fn fetch_stations_from_api() -> Result<Vec<String>> {
        let base_url = env::var("SeptumURL").context("SeptumURL not set, cannot fetch stations")?;
        let result: Stations = reqwest::blocking::get(format!("{}/stations", base_url))?.json()?;
        let stations: Vec<String> = result
            .0
            .iter()
            .map(|station| {
                if station.parameter == station.station_name {
                    station.parameter.to_owned()
                } else {
                    format!("{} ({})", station.parameter, station.station_name)
                }
            })
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

        let metadata = fs::metadata(app_dirs.cache_dir.join("stations"))?;
        let time = metadata.modified().context("Unsupported platform")?;
        let diff = SystemTime::now()
            .duration_since(time)
            .context("Time went backwards")?
            .as_secs();
        if diff > SECONDS_IN_WEEK {
            return Err(anyhow!("Station name cache too old"));
        }

        let mut f = BufReader::new(
            OpenOptions::new()
                .read(true)
                .open(app_dirs.cache_dir.join("stations"))?,
        );

        let stations: Vec<String> = deserialize_from(&mut f)?;
        Ok(stations)
    }
}

const SECONDS_IN_WEEK: u64 = 604800;

const FALLBACK_STATIONS: [&'static str; 154] = [
    "9th St (9th Street)",
    "30th Street Station (30th Street)",
    "49th St (49th Street)",
    "Airport Terminal A",
    "Airport Terminal B",
    "Airport Terminal C-D",
    "Airport Terminal E-F",
    "Allegheny",
    "Allen Lane",
    "Ambler",
    "Angora",
    "Ardmore",
    "Ardsley",
    "Bala",
    "Berwyn",
    "Bethayres",
    "Bridesburg",
    "Bristol",
    "Bryn Mawr",
    "Carpenter",
    "Chalfont",
    "Chelten Avenue",
    "Cheltenham",
    "Chester TC (Chester Transportation Center)",
    "Chestnut Hill East",
    "Chestnut Hill West",
    "Churchmans Crossing (Churchmans Crossing, DE)",
    "Claymont (Claymont, DE)",
    "Clifton-Aldan",
    "Colmar",
    "Conshohocken",
    "Cornwells Heights",
    "Crestmont",
    "Croydon",
    "Crum Lynne",
    "Curtis Park",
    "Cynwyd",
    "Daylesford",
    "Darby",
    "Delaware Valley College",
    "Devon",
    "Downingtown",
    "Doylestown",
    "East Falls",
    "Eastwick Station (Eastwick)",
    "Eddington",
    "Eddystone",
    "Elkins Park",
    "Elm St (Elm Street-Norristown)",
    "Elwyn Station (Elwyn)",
    "Exton",
    "Fern Rock TC (Fern Rock Transportation Center)",
    "Fernwood (Fernwood-Yeadon)",
    "Folcroft",
    "Forest Hills",
    "Ft Washington (Fort Washington)",
    "Fortuna",
    "Fox Chase",
    "Germantown",
    "Gladstone",
    "Glenolden",
    "Glenside",
    "Gravers",
    "Gwynedd Valley",
    "Hatboro",
    "Haverford",
    "Highland Ave (Highland Avenue)",
    "Highland",
    "Holmesburg Jct (Holmesburg Junction)",
    "Ivy Ridge",
    "Jefferson Station",
    "Jenkintown-Wyncote",
    "Langhorne",
    "Lansdale",
    "Lansdowne",
    "Lawndale",
    "Levittown",
    "Link Belt",
    "Main St (Main Street-Norristown)",
    "Malvern",
    "Manayunk",
    "Marcus Hook",
    "Meadowbrook",
    "Media",
    "Melrose Park",
    "Merion",
    "Miquon",
    "Morton",
    "Moylan-Rose Valley",
    "Mt Airy (Mt. Airy)",
    "Narberth",
    "Neshaminy Falls",
    "New Britain",
    "Newark (Newark Station)",
    "Noble (Noble Station)",
    "Norristown TC (Norristown Transportation Center)",
    "North Broad St (North Broad)",
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
    "Penn Medicine Station (Penn Medicine Station (University City))",
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
    "Tacony",
    "Temple U (Temple University)",
    "Thorndale",
    "Torresdale",
    "Trenton (Trenton Transit Center)",
    "Trevose",
    "Tulpehocken",
    "Upsal",
    "Villanova",
    "Wallingford",
    "Warminster",
    "Washington Lane",
    "Wawa",
    "Wayne Jct (Wayne Junction)",
    "Wayne Station (Wayne)",
    "West Trenton (West Trenton, NJ)",
    "Whitford",
    "Willow Grove",
    "Wilmington (Wilmington, DE)",
    "Wissahickon",
    "Wister",
    "Woodbourne",
    "Wyndmoor",
    "Wynnefield Avenue",
    "Wynnewood",
    "Yardley",
];
