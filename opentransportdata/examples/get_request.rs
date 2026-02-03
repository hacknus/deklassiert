use opentransportdata::{get_train_formation, parse_formation_short_string};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv::dotenv().ok();

    let token = std::env::var("FORMATION_TOKEN").expect("FORMATION_TOKEN not set");

    let formation = get_train_formation(739,2026,2,2, &token).unwrap();

    println!("Train number: {}", formation.train_meta_information.train_number);

    for stop in formation.formations_at_scheduled_stops {
        println!(
            "Stop: {} (track {})",
            stop.scheduled_stop.stop_point.name, stop.scheduled_stop.track
        );

        let parsed =
            parse_formation_short_string(&stop.formation_short.formation_short_string);

        for vehicle in parsed {
            println!("{:#?}", vehicle);
        }
    }

    Ok(())
}