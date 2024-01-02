// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod task; // This makes the content of task.rs available as the task module

use task::Task;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!!!", name)
}

#[tauri::command]
fn get_tasks() -> Vec<Task> {
    let task1: Task = Task::new("Task 1", "This is a sample task description.");
    let task2: Task = Task::new("Task 2", "This is a sample task description.");

    let tasks: Vec<Task> = vec![task1, task2];

    tasks
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_tasks, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
