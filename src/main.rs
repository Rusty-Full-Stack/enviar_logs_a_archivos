use enviar_logs_a_archivos::mostrar_un_info;
use log::{debug, error, info, trace, warn};
// use std::time::SystemTime;
use chrono::prelude::*;

fn setup_logs() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            // Aca definimos la estructura de nuestros logs
            // practicametne puedes hacerlo como creas conveniente.
            out.finish(format_args!(
                "[{} {} {}] {}", // Aca definimos la estructura de nuestro log, en este caso tendremos {hora y fecha} {nivel del log} {ubicacion} {mensaje}
                // humantime::format_rfc3339_seconds(SystemTime::now()),
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), // Este es el primer argumento, la hora local
                record.level(), // El segundo element de la estructura de los logs es el nivel
                record.target(), // El tercer elemento es la ubicacion
                message         // por ultimo agregamos el mensaje
            ))
        })
        .level(log::LevelFilter::Trace) // Definimos que el nivel minimo de logs que queremos registrar es Trace
        .chain(std::io::stdout()) // Aca indicamos que queremos que los logs se muestren en la terminal mediante el stdout
        .chain(fern::log_file("logs.log")?) // Aca tambien indicamos que queremos los mismos logs pero en un archivo llamado logs.log
        .apply()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logs()?;

    error!("Este es un error");

    warn!("Hola soy un warning!");

    info!("Yo soy un info");

    debug!("Mensaje de debug");

    trace!("El de prioridad bajita");

    mostrar_un_info();

    Ok(())
}
