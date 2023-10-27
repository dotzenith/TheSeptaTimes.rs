mod api;
mod parse;
mod stations;

use crate::api::SeptaApi;
use crate::parse::Parse;

fn main() {
    let api = SeptaApi::new();

    // let result = api.next_to_arrive("30th Street Station", "Cornwells Heights", Some(5));
    // match result {
    //     Ok(res) => {
    //         for train in res.parse().iter() {
    //             println!("{train}");
    //         }
    //     },
    //     Err(_) => println!("Something errored out"),
    // };

    let result = api.arrivals("30th Street Station", Some(5));
    match result {
        Ok(res) => {
            for train in res.parse().iter() {
                println!("{train}");
            }
        },
        Err(_) => println!("Something errored out"),
    };

    // let result = api.train_schedule("729");
    // match result {
    //     Ok(res) => {
    //         for train in res.parse().iter() {
    //             println!("{train}");
    //         }
    //     },
    //     Err(_) => println!("Something errored out"),
    // };
}
