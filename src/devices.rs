use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use rand::{RngExt};
use std::thread;
use std::time::Duration;
use crate::devices::Magnitude::{Temperature, Voltage};

pub trait Sensor {
    fn name(&self) -> String;
    fn read_value(&self) -> f64;
    fn type_of_measurement(&self) -> Magnitude;
}

pub enum Magnitude {
    Temperature,
    Voltage,
}
pub struct Thermometer {
    id: String,
}

pub struct Voltmeter {
    id: String,
}

impl Thermometer {
    pub fn new(id: String) -> Thermometer {
        Thermometer {
            id,
        }
    }
}

impl Voltmeter {
    pub fn new(id: String) -> Voltmeter {
        Voltmeter {
            id,
        }
    }
}

impl Sensor for Thermometer {
    fn name(&self) -> String {
        self.id.clone()
    }
    fn read_value(&self) -> f64 {
        let value = rand::rng().random_range(-5.0..=50.0);
        value
    }
    fn type_of_measurement(&self) -> Magnitude {
        Temperature
    }
}

impl Sensor for Voltmeter {
    fn name(&self) -> String {
        self.id.clone()
    }
    fn read_value(&self) -> f64 {
        let value = rand::rng().random_range(200.0..=250.0);
        value
    }
    fn type_of_measurement(&self) -> Magnitude {
        Voltage
    }
}

pub fn show_measurements(l: &Vec<Box<dyn Sensor>>) {

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("\n[!] Ctrl+C detectado. Preparando salida segura...");
        r.store(false, Ordering::SeqCst);
    }).expect("Error al configurar el manejador de Ctrl+C");

    println!("Presiona Ctrl+C para detener el monitoreo.");
    println!("-----------------------------------------------");
    thread::sleep(Duration::from_secs(2));

    while running.load(Ordering::SeqCst) {
        if l.is_empty() {
            println!("No se han encontrado sensores instalados para la evaluación de sus mediciones.\n");
            break;
        } else {
            for d in l {

                let value = d.read_value();

                match d.type_of_measurement() {
                    Temperature => {

                        if value < 0.0 {
                            println!("[{}]: {:.2}°C (¡ALERTA DE CONGELAMIENTO!)", d.name(), value);
                        } else if value > 35.0 {
                            println!("[{}]: {:.2}°C (¡ALERTA DE SOBRECALENTAMIENTO!)", d.name(), value);
                        } else {
                            println!("[{}]: {:.2}°C (Normal)", d.name(), value);
                        }
                    },
                    Voltage => {
                        if value > 240.0 {
                            println!("[{}]: {:.2}V (¡ALERTA DE SOBRETENSIÓN!!)", d.name(), value);
                        } else {
                            println!("[{}]: {:.2}V (Normal)", d.name(), value);
                        }
                    }
                }
            }
            println!();
            thread::sleep(Duration::from_secs(2));
        }
    }
}
