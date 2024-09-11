#[derive(Debug, Clone)]
pub struct Resource {
    pub name: String,
    pub allowed_roles: Vec<String>, // Roles allowed to access this resource
}

impl Resource {
    pub fn new(name: &str, allowed_roles: Vec<String>) -> Self {
        Resource {
            name: name.to_string(),
            allowed_roles,
        }
    }

    // Check if a role is allowed to access this resource
    pub fn is_access_allowed(&self, role: &str) -> bool {
        self.allowed_roles.contains(&role.to_string())
    }
}

pub struct ResourceGroup {
    pub group_name: String,
    pub resources: Vec<Resource>,
}

impl ResourceGroup {
    pub fn new(group_name: &str, resources: Vec<Resource>) -> Self {
        ResourceGroup {
            group_name: group_name.to_string(),
            resources,
        }
    }

    // Check if a role has access to any resource in the group
    pub fn is_access_allowed(&self, role: &str) -> bool {
        self.resources.iter().any(|resource| resource.is_access_allowed(role))
    }
}
