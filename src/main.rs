use std::thread;
use std::time::Duration;

mod devices;
use crate::devices::{Sensor, Thermometer, Voltmeter};
use crate::devices::show_measurements;

fn main() {

    let mut sensors_list: Vec<Box<dyn Sensor>> = Vec::new();

    loop {
        menu();

        let mut option = String::new();

        println!("\nIngrese una opción: ");
        std::io::stdin().read_line(&mut option).expect("Failed to read line");
        let option = option.trim();

        match option {
            "1" => {
                println!("Crea el identificador del termómetro: ");
                let mut thermometer_id = String::new();
                std::io::stdin().read_line(&mut thermometer_id).expect("Failed to read line");
                let thermometer_id = thermometer_id.trim().to_string();

                let thermometer = Thermometer::new(thermometer_id.clone());
                sensors_list.push(Box::new(thermometer));

                println!("------------------------------------------");
                println!("Se ha instalado el termómetro {} con éxito", thermometer_id);
                println!("------------------------------------------");
            }
            "2" => {
                println!("Crea el identificador del voltímetro: ");
                let mut voltmeter_id = String::new();
                std::io::stdin().read_line(&mut voltmeter_id).expect("Failed to read line");
                let voltmeter_id = voltmeter_id.trim().to_string();

                let voltmeter = Voltmeter::new(voltmeter_id.clone());
                sensors_list.push(Box::new(voltmeter));

                println!("------------------------------------------");
                println!("Se ha instalado el voltímetro {} con éxito", voltmeter_id);
                println!("------------------------------------------");
            }
            "3" => {
                monitoring();
                show_measurements(&sensors_list);
            }
            "4" => {
                println!("...saliendo del centro de mando");
                thread::sleep(Duration::from_secs(2));
                println!("SE HA COMPLETADO LA SALIDA CON ÉXITO"); break;
            }
            _ => {
                println!("El valor ingresado no es válido. Por favor, vuelva a intentarlo");
                continue;
            }
        }
    }
}

fn menu() {
    println!("\n------ CENTRO DE MANDO: IoT ------
    1. Instalación del termómetro
    2. Instalación del voltímetro
    3. Verificación del funcionamiento de sensores
    4. Salir")
}

fn monitoring() {
    println!("\nINICIANDO MONITOREO...\n-----------------------------------------------");
    thread::sleep(Duration::from_secs(1));
}