use std::time::Duration;

mod client;
mod geo;

use client::fetch_iss_position;
use geo::Observer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let me = Observer::new(-34.728,  -58.428);

    let mut interval = tokio::time::interval(Duration::from_secs(2));

    println!("Iniciando seguimiento en vivo de la ISS (Ctrl + C para salir)...\n");

    loop {
        // Pausa la ejecución hasta el próximo pulso de 2 segundos sin bloquear el hilo
        interval.tick().await;

        match fetch_iss_position().await {
            Ok(iss) => {
                let distance = me.distance_to(&iss);

                // Secuencia ANSI "\x1B[2J\x1B[1;1H" para limpiar la terminal en cada tick
                print!("\x1B[2J\x1B[1;1H");

                println!("=== ISS Tracker (En Vivo) ===");
                println!("Satelite:     {}", iss.name);
                println!("Latitud:      {:.4}°", iss.latitude);
                println!("Longitud:     {:.4}°", iss.longitude);
                println!("Altitud:      {:.2} km", iss.altitude);
                println!("Velocidad:    {:.2} km/h", iss.velocity);
                println!("Visibilidad:  {}", iss.visibility);
                println!("-----------------------------");
                println!("Distancia:    {:.2} km", distance);
            }
            Err(e) => {
                eprintln!("Error al consultar la telemetria: {}", e);
            }
        }
    }

}   
