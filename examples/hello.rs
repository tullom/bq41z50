use bq41z50::Bq41z50;
use embedded_batteries_async::smart_battery::SmartBattery;

#[tokio::main]
async fn main() {
    println!("Hello world!");

    let hal = pico_de_gallo_hal::Hal::new();

    let i2c = hal.i2c();

    let mut bq41z50 = Bq41z50::new(i2c);

    if let Ok(serial) = bq41z50.device.serial_number().read_async().await {
        println!("Serial Num: {:?}", serial);
    } else {
        eprintln!("Error getting serial number from gauge");
    }

    if let Ok(serial) = bq41z50.serial_number().await {
        println!("Serial Num: {:?}", serial);
    } else {
        eprintln!("Error getting serial number from gauge");
    }

    let chem_id = bq41z50.device.mac_chem_id().dispatch_out_async().await.unwrap();

    println!("Chem ID: {:?}", chem_id);

    if let Ok(temp) = bq41z50.temperature_celsius().await {
        println!("Temperature: {:.2} C", temp.to_degrees());
    } else {
        eprintln!("Error getting temperature from gauge");
    }
    if let Ok(temp) = bq41z50.temperature_fahrenheit().await {
        println!("Temperature: {:.2} F", temp.to_degrees());
    } else {
        eprintln!("Error getting temperature from gauge");
    }

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        if let Ok(temp) = bq41z50.temperature_celsius().await {
            println!("Temperature: {:.2} C", temp.to_degrees());
        } else {
            eprintln!("Error getting temperature from gauge");
        }
        if let Ok(voltage) = bq41z50.voltage().await {
            println!("Voltage: {:?}", voltage);
        } else {
            eprintln!("Error getting voltage from gauge");
        }
        if let Ok(cell_voltage) = bq41z50.device.cell_voltage_1().read_async().await {
            println!("voltage 1: {:?}", cell_voltage);
        } else {
            eprintln!("Error getting cell voltage 1 from gauge");
        }
        if let Ok(cell_voltage) = bq41z50.device.cell_voltage_2().read_async().await {
            println!("voltage 2: {:?}", cell_voltage);
        } else {
            eprintln!("Error getting cell voltage 2 from gauge");
        }
        if let Ok(cell_voltage) = bq41z50.device.cell_voltage_3().read_async().await {
            println!("voltage 3: {:?}", cell_voltage);
        } else {
            eprintln!("Error getting cell voltage 3 from gauge");
        }
        if let Ok(cell_voltage) = bq41z50.device.cell_voltage_4().read_async().await {
            println!("voltage 4: {:?}", cell_voltage);
        } else {
            eprintln!("Error getting cell voltage 4 from gauge");
        }
    }

    // bq41z50
    //     .auth_send_challenge(&[0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF])
    //     .await;

    // let sig = bq41z50.auth_read_signature().await;
}
