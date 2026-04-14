use crate::protocol::{AckMessage, RegisterMessage, ResultMessage, TaskMessage};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const MAX_ITER: u32 = 1000;
const TOTAL_WORKERS: usize = 8;

type SharedResults = Arc<Mutex<HashMap<String, ResultMessage>>>;

pub fn run() {
    let addr = "0.0.0.0:9000";
    let listener = TcpListener::bind(addr).expect("No se pudo abrir el puerto 9000");

    println!("Coordinador escuchando en {}", addr);

    let results: SharedResults = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let results_clone = Arc::clone(&results);

                thread::spawn(move || {
                    println!("Nueva conexión recibida");
                    handle_client(stream, results_clone);
                });
            }
            Err(e) => {
                eprintln!("Error aceptando conexión: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, results: SharedResults) {
    let mut reader = BufReader::new(stream.try_clone().expect("No se pudo clonar stream"));
    let mut line = String::new();

    match reader.read_line(&mut line) {
        Ok(_) => {
            println!("Mensaje recibido: {}", line.trim());

            let register: Result<RegisterMessage, _> = serde_json::from_str(line.trim());

            match register {
                Ok(msg) => {
                    println!(
                        "Worker registrado: {} desde host {}",
                        msg.worker_id, msg.host_id
                    );

                    let ack = AckMessage {
                        r#type: "ack".to_string(),
                        message: "worker registered".to_string(),
                    };

                    let ack_json =
                        serde_json::to_string(&ack).expect("No se pudo serializar ACK");

                    stream
                        .write_all(format!("{}\n", ack_json).as_bytes())
                        .expect("No se pudo enviar ACK");

                    println!("ACK enviado");

                    let (start_row, end_row) = assign_rows(&msg.worker_id);

                    let task = TaskMessage {
                        r#type: "task".to_string(),
                        task_id: extract_worker_number(&msg.worker_id),
                        start_row,
                        end_row,
                        width: WIDTH,
                        height: HEIGHT,
                        max_iter: MAX_ITER,
                        x_min: -2.0,
                        x_max: 1.0,
                        y_min: -1.5,
                        y_max: 1.5,
                    };

                    let task_json =
                        serde_json::to_string(&task).expect("No se pudo serializar task");

                    stream
                        .write_all(format!("{}\n", task_json).as_bytes())
                        .expect("No se pudo enviar task");

                    println!(
                        "TASK enviada a {} para filas {}-{}",
                        msg.worker_id, start_row, end_row
                    );

                    let mut result_line = String::new();
                    reader
                        .read_line(&mut result_line)
                        .expect("No se pudo leer resultado");

                    println!("Resultado recibido de {}", msg.worker_id);

                    let result: Result<ResultMessage, _> =
                        serde_json::from_str(result_line.trim());

                    match result {
                        Ok(res) => {
                            println!(
                                "Resultado válido de {} para filas {}-{}",
                                res.worker_id, res.start_row, res.end_row
                            );
                            println!("Cantidad de pixeles recibidos: {}", res.pixels.len());

                            {
                                let mut map = results.lock().expect("No se pudo bloquear results");
                                map.insert(res.worker_id.clone(), res);

                                println!("Bloques recibidos hasta ahora: {}/{}", map.len(), TOTAL_WORKERS);

                                if map.len() == TOTAL_WORKERS {
                                    println!("Todos los bloques fueron recibidos. Integrando resultado final...");
                                    write_final_output(&map);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("No se pudo parsear result: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("No se pudo parsear mensaje de registro: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error leyendo del worker: {}", e);
        }
    }
}

fn extract_worker_number(worker_id: &str) -> u32 {
    worker_id
        .split('-')
        .last()
        .unwrap_or("1")
        .parse::<u32>()
        .unwrap_or(1)
}

fn assign_rows(worker_id: &str) -> (u32, u32) {
    let worker_number = extract_worker_number(worker_id);

    let rows_per_worker = 100;
    let start_row = (worker_number - 1) * rows_per_worker;
    let end_row = start_row + rows_per_worker - 1;

    (start_row, end_row)
}

fn write_final_output(results: &HashMap<String, ResultMessage>) {
    let mut ordered_results: Vec<&ResultMessage> = results.values().collect();
    ordered_results.sort_by_key(|r| r.start_row);

    let mut final_pixels: Vec<u32> = Vec::new();

    for result in ordered_results {
        final_pixels.extend_from_slice(&result.pixels);
    }

    let mut file = File::create("mandelbrot_result.txt")
        .expect("No se pudo crear el archivo de salida");

    writeln!(file, "width={}", WIDTH).unwrap();
    writeln!(file, "height={}", HEIGHT).unwrap();
    writeln!(file, "max_iter={}", MAX_ITER).unwrap();
    writeln!(file, "total_pixels={}", final_pixels.len()).unwrap();
    writeln!(file, "pixels={:?}", final_pixels).unwrap();

    println!("Archivo final generado: mandelbrot_result.txt");
}