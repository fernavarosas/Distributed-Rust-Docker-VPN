use crate::mandelbrot::compute_block;
use crate::protocol::{AckMessage, RegisterMessage, ResultMessage, TaskMessage};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub fn run(worker_id: String, coord_addr: String) {
    println!("Iniciando worker...");
    println!("worker_id: {}", worker_id);
    println!("coordinador: {}", coord_addr);

    let mut stream =
        TcpStream::connect(&coord_addr).expect("No se pudo conectar al coordinador");

    let register = RegisterMessage {
        r#type: "register".to_string(),
        worker_id: worker_id.clone(),
        host_id: "local-host".to_string(),
    };

    let register_json =
        serde_json::to_string(&register).expect("No se pudo serializar registro");

    stream
        .write_all(format!("{}\n", register_json).as_bytes())
        .expect("No se pudo enviar registro");

    println!("Registro enviado al coordinador");

    let mut reader = BufReader::new(stream.try_clone().expect("No se pudo clonar stream"));

    let mut ack_line = String::new();
    reader
        .read_line(&mut ack_line)
        .expect("No se pudo leer ACK");

    let ack: AckMessage =
        serde_json::from_str(ack_line.trim()).expect("No se pudo parsear ACK");

    println!("Respuesta del coordinador: {}", ack.message);

    let mut task_line = String::new();
    reader
        .read_line(&mut task_line)
        .expect("No se pudo leer TASK");

    let task: TaskMessage =
        serde_json::from_str(task_line.trim()).expect("No se pudo parsear TASK");

    println!(
        "Procesando filas {}-{} con width={} height={}",
        task.start_row, task.end_row, task.width, task.height
    );

    let pixels = compute_block(
        task.start_row,
        task.end_row,
        task.width,
        task.height,
        task.max_iter,
        task.x_min,
        task.x_max,
        task.y_min,
        task.y_max,
    );

    let result = ResultMessage {
        r#type: "result".to_string(),
        task_id: task.task_id,
        worker_id: worker_id.clone(),
        start_row: task.start_row,
        end_row: task.end_row,
        pixels,
    };

    let result_json =
        serde_json::to_string(&result).expect("No se pudo serializar RESULT");

    stream
        .write_all(format!("{}\n", result_json).as_bytes())
        .expect("No se pudo enviar RESULT");

    println!("RESULT enviado al coordinador");
}