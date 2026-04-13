mod coordinator;
mod worker;
mod protocol;
mod mandelbrot;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso:");
        eprintln!("  cargo run -- coordinator");
        eprintln!("  cargo run -- worker <worker_id> <coord_addr>");
        return;
    }

    match args[1].as_str() {
        "coordinator" => {
            coordinator::run();
        }
        "worker" => {
            if args.len() < 4 {
                eprintln!("Uso: cargo run -- worker <worker_id> <coord_addr>");
                return;
            }

            let worker_id = args[2].clone();
            let coord_addr = args[3].clone();
            worker::run(worker_id, coord_addr);
        }
        _ => {
            eprintln!("Modo no válido. Usa 'coordinator' o 'worker'.");
        }
    }
}