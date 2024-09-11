#[derive(Debug, Clone)]
pub struct Ownable {
    owner: String, // Stores the current owner's name
}

impl Ownable {
    // Initializes a new Ownable resource with a specific owner
    pub fn new(owner: &str) -> Self {
        Ownable {
            owner: owner.to_string(),
        }
    }

    // Transfers ownership to a new owner
    pub fn transfer_ownership(&mut self, new_owner: &str) {
        self.owner = new_owner.to_string();
    }

    // Retrieves the current owner
    pub fn get_owner(&self) -> &str {
        &self.owner
    }
}

// Unit tests for the Ownable struct
#[cfg(test)]
mod tests {
    use super::*;

    // Test initial ownership
    #[test]
    fn test_initial_ownership() {
        let ownable = Ownable::new("alice");
        assert_eq!(ownable.get_owner(), "alice");
    }

    // Test ownership transfer
    #[test]
    fn test_transfer_ownership() {
        let mut ownable = Ownable::new("alice");
        assert_eq!(ownable.get_owner(), "alice");

        ownable.transfer_ownership("bob");
        assert_eq!(ownable.get_owner(), "bob");
    }

    // Test multiple ownership transfers
    #[test]
    fn test_multiple_transfers() {
        let mut ownable = Ownable::new("alice");
        ownable.transfer_ownership("bob");
        assert_eq!(ownable.get_owner(), "bob");

        ownable.transfer_ownership("charlie");
        assert_eq!(ownable.get_owner(), "charlie");

        ownable.transfer_ownership("dave");
        assert_eq!(ownable.get_owner(), "dave");
    }
}
