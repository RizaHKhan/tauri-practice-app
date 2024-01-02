use serde::Serialize;

#[derive(Serialize)]
pub struct Task {
    title: String,
    description: String,
}

impl Task {
    pub fn new(title: &str, description: &str) -> Task {
        Task {
            title: title.to_string(),
            description: description.to_string(),
        }
    }
}
