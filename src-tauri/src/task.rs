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

#[test]
fn test_task_creation() {
    let title = "Sample Title";
    let description = "Sample Description";
    let task = Task::new(title, description);

    assert_eq!(task.title, title);
    assert_eq!(task.description, description);
}
