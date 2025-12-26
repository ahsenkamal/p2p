pub struct State {
    username: String,
    to: String,
}

impl State {
    pub fn new(username: String) -> Self {
        Self {
            username,
            to: "BROADCAST".to_string(),
        }
    }

    pub fn change_to(&mut self, to: String) {
        self.to = to;
    }

    pub fn get_to(&self) -> &str {
        &self.to
    }
}