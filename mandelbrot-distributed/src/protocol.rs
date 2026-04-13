use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterMessage {
    pub r#type: String,
    pub worker_id: String,
    pub host_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AckMessage {
    pub r#type: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskMessage {
    pub r#type: String,
    pub task_id: u32,
    pub start_row: u32,
    pub end_row: u32,
    pub width: u32,
    pub height: u32,
    pub max_iter: u32,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultMessage {
    pub r#type: String,
    pub task_id: u32,
    pub worker_id: String,
    pub start_row: u32,
    pub end_row: u32,
    pub pixels: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DoneMessage {
    pub r#type: String,
    pub task_id: u32,
    pub worker_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    pub r#type: String,
    pub worker_id: String,
    pub task_id: u32,
    pub message: String,
}