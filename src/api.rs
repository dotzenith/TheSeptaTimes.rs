use crate::parse::{ArrivalResult, NextToArriveResult, TrainScheduleResult};
use reqwest::blocking::get;
use simsearch::SimSearch;
use std::error::Error;

#[derive(Debug)]
pub struct SeptaApi {
    base_url: &'static str,
    stations: Vec<&'static str>,
    search_engine: SimSearch<usize>,
}

impl SeptaApi {
    pub fn next_to_arrive(
        &self,
        from: &str,
        to: &str,
        num: Option<u8>,
    ) -> Result<NextToArriveResult, Box<dyn Error>> {
        let request_url = format!(
            "{}/NextToArrive/index.php?req1={}&req2={}&req3={}",
            self.base_url,
            from,
            to,
            num.unwrap_or(5)
        );
        let result: NextToArriveResult = get(request_url)?.json()?;
        Ok(result)
    }

    pub fn arrivals(&self, name: &str, num: Option<u8>) -> Result<ArrivalResult, Box<dyn Error>> {
        let request_url = format!(
            "{}/Arrivals/index.php?station={}&results={}",
            self.base_url,
            name,
            num.unwrap_or(5)
        );
        let result: ArrivalResult = get(request_url)?.json()?;
        Ok(result)
    }

    pub fn train_schedule(&self, num: &str) -> Result<TrainScheduleResult, Box<dyn Error>> {
        let request_url = format!("{}/RRSchedules/index.php?req1={}", self.base_url, num);
        let result: TrainScheduleResult = get(request_url)?.json()?;
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
            stations: vec![
                "9th St",
                "Chalfont",
                "30th street station",
                "49th st",
                "airport terminal a",
                "airport terminal b",
                "airport terminal c-d",
                "airport terminal e-f",
                "allegheny",
                "allen lane",
                "ambler",
                "angora",
                "ardmore",
                "ardsley",
                "bala",
                "berwyn",
                "bethayres",
                "bridesburg",
                "bristol",
                "bryn mawr",
                "carpenter",
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
            ],
            search_engine: SimSearch::new(),
        };
        api.populate_search_engine();
        api
    }
}
