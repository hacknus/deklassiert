use opentransportdata::fetch_train_numbers;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv::dotenv().ok();

    let token = std::env::var("OJP_TOKEN").expect("set OJP_TOKEN env var");
    let trains = fetch_train_numbers(&token)?;
    println!("{:#?}", trains);
    Ok(())
}