pub struct Task {
    Title: String,
    Description: String,
}

impl Task {
    pub fn new(title: String, description: String) -> Self {
        self.Title = title;
        self.Description = description;
    }
}
