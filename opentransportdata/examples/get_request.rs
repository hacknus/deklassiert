use opentransportdata::{get_train_formation, parse_formation_for_stop};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv::dotenv().ok();

    let token = std::env::var("FORMATION_TOKEN").expect("FORMATION_TOKEN not set");

    let formation = get_train_formation(808,2026,2,7, &token).unwrap();

    println!("Train number: {}", formation.train_meta_information.train_number);
    // dbg!(&formation);

    for (i, stop) in formation.formations_at_scheduled_stops.iter().enumerate() {
        println!(
            "Stop: {} (track {})",
            stop.scheduled_stop.stop_point.name, stop.scheduled_stop.track
        );

        let parsed = parse_formation_for_stop(&formation, i);

        for vehicle in parsed {
            println!("{:#?}", vehicle);
        }
    }

    Ok(())
}
