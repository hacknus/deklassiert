use opentransportdata::{
    parse_formation_for_stop, parse_formation_json,
};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string("test_data/test_response.json")?;
    let formation = parse_formation_json(&json_data)?;

    println!(
        "Train number: {}",
        formation.train_meta_information.train_number
    );

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
