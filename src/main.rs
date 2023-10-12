mod api;
mod parse;

use crate::api::SeptaApi;
use crate::parse::Parse;

fn main() {
    let api = SeptaApi::new();
    // println!("{:?}", api.next_to_arrive("30th Street Station", "Cornwells Heights", None).unwrap()[0]);
    // println!("{:?}", api.arrivals("30th Street Station", None).unwrap());
    // println!("{:?}", api.train_schedule("731").unwrap()[0]);
    println!("{:?}", api.next_to_arrive("30th Street Station", "Cornwells Heights", Some(5)).unwrap().parse());
}
