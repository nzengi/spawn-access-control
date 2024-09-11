use crate::role::Role;
use crate::resource::Resource;

pub struct AccessControl {
    pub user_roles: Vec<Role>, // List of roles assigned to the user
}

impl AccessControl {
    pub fn new() -> Self {
        AccessControl {
            user_roles: Vec::new(),
        }
    }

    // Add a role to the user
    pub fn add_role(&mut self, role: Role) {
        self.user_roles.push(role);
    }

    // Check if the user has access to a specific resource based on their role
    pub fn has_access(&self, resource: &Resource) -> bool {
        for role in &self.user_roles {
            if resource.is_access_allowed(&role.name) {
                return true;
            }
        }
        false
    }

    // Check if the user has a specific role
    pub fn has_role(&self, role_name: &str) -> bool {
        self.user_roles.iter().any(|role| role.name == role_name)
    }
}
