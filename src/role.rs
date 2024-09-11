#[derive(Debug, Clone)]
pub struct Role {
    pub name: String,
    pub parent_role: Option<Box<Role>>, // Optional parent role
}

impl Role {
    pub fn new(name: &str, parent: Option<Box<Role>>) -> Self {
        Role {
            name: name.to_string(),
            parent_role: parent,
        }
    }

    // Check if the current role is higher or equal in the hierarchy
    pub fn is_higher_or_equal(&self, other: &Role) -> bool {
        if self.name == other.name {
            true
        } else if let Some(ref parent) = self.parent_role {
            parent.is_higher_or_equal(other)
        } else {
            false
        }
    }
}

pub struct RoleDelegation {
    pub delegator: String,
    pub delegatee: String,
    pub role: Role,
    pub expiration: u64, // Delegation expiration time
}

impl RoleDelegation {
    pub fn new(delegator: &str, delegatee: &str, role: Role, expiration: u64) -> Self {
        RoleDelegation {
            delegator: delegator.to_string(),
            delegatee: delegatee.to_string(),
            role,
            expiration,
        }
    }

    // Check if the delegation is still valid
    pub fn is_valid(&self) -> bool {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() < self.expiration
    }
}
