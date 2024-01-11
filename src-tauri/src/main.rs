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
    let mut tasks: Vec<Task> = Vec::new();

    for n in 1..10 {
        let task_name: String = format!("Task {}", n);
        let task_description: String = format!("{}", "Lorem ipsum");
        tasks.push(Task::new(&task_name, &task_description));
    }

    tasks
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_tasks, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
