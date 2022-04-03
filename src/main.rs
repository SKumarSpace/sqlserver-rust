use async_std::net::TcpStream;
use dotenv::dotenv;
use std::env;
use tiberius::{Client, Config};

struct Vehicle {
    year: i32,
    make: String,
    model: String,
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let vehicles = get_vehicles().await?;

    for vehicle in vehicles {
        println!("{} {} {}", vehicle.year, vehicle.make, vehicle.model);
    }

    Ok(())
}

async fn get_vehicles() -> anyhow::Result<Vec<Vehicle>> {
    let connection_string = env::var("CONNECTION_STRING").expect("CONNECTION_STRING must be set");
    // println!("{}", connection_string);

    let config = Config::from_ado_string(&connection_string)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;

    let mut vehicles = Vec::<Vehicle>::new();

    let rows = client
        .query("SELECT Year, Make, Model FROM Vehicles", &[&1i32])
        .await?
        .into_first_result()
        .await?;

    for row in rows {
        let year: i32 = row.get("Year").unwrap();
        let make: &str = row.get("Make").unwrap();
        let model: &str = row.get("Model").unwrap();

        vehicles.push(Vehicle {
            year,
            make: make.to_string(),
            model: model.to_string(),
        });
    }

    Ok(vehicles)
}
