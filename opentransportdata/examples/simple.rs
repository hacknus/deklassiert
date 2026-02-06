use opentransportdata::{
    get_vehicle_information, parse_formation_json, parse_formation_short_string,
};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string("test_data/test_response.json")?;
    let formation = parse_formation_json(&json_data)?;

    println!(
        "Train number: {}",
        formation.train_meta_information.train_number
    );

    let vehicle_information = get_vehicle_information(&formation);

    for stop in formation.clone().formations_at_scheduled_stops {
        println!(
            "Stop: {} (track {})",
            stop.scheduled_stop.stop_point.name, stop.scheduled_stop.track
        );

        let parsed = parse_formation_short_string(
            &stop.formation_short.formation_short_string,
            &vehicle_information,
        );

        for vehicle in parsed {
            println!("{:#?}", vehicle);
        }
    }

    Ok(())
}
