#[derive(Debug, Clone)]
pub struct Permission {
    pub name: String,  // read, write, delete
    pub resource: String,  // Associated resource
    pub condition: Option<String>,  // Optional condition (e.g. "only during office hours")
}

impl Permission {
    pub fn is_granted(&self, condition: Option<String>) -> bool {
        if let Some(ref c) = self.condition {
            if condition.is_some() && &condition.unwrap() == c {
                return true;
            }
            return false;
        }
        true
    }
}
