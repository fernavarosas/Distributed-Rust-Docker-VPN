use crate::protocol::{AckMessage, RegisterMessage, ResultMessage, TaskMessage};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

pub fn run() {
    let addr = "0.0.0.0:9000";
    let listener = TcpListener::bind(addr).expect("No se pudo abrir el puerto 9000");

    println!("Coordinador escuchando en {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Nueva conexión recibida");
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Error aceptando conexión: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
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

                    let task = TaskMessage {
                        r#type: "task".to_string(),
                        task_id: 1,
                        start_row: 0,
                        end_row: 9,
                        width: 10,
                        height: 10,
                        max_iter: 100,
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

                    println!("TASK enviada");

                    let mut result_line = String::new();
                    reader
                        .read_line(&mut result_line)
                        .expect("No se pudo leer resultado");

                    println!("Resultado recibido: {}", result_line.trim());

                    let result: Result<ResultMessage, _> =
                        serde_json::from_str(result_line.trim());

                    match result {
                        Ok(res) => {
                            println!(
                                "Resultado válido de {} para filas {}-{}",
                                res.worker_id, res.start_row, res.end_row
                            );
                            println!("Cantidad de pixeles recibidos: {}", res.pixels.len());
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