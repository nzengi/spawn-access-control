use crate::{Role, Resource, AccessControl, RateLimiter}; // Permission kaldırıldı, kullanılmıyordu

pub struct AccessManager {
    pub users: Vec<(String, AccessControl)>, // Stores username and their access control
    pub rate_limiter: RateLimiter, // Rate limiter to prevent excessive requests
}

impl AccessManager {
    pub fn new() -> Self {
        AccessManager {
            users: Vec::new(),
            rate_limiter: RateLimiter::new(100, 60), // Example rate limiting: 100 requests per minute
        }
    }

    // Add a user with their associated access control
    pub fn add_user(&mut self, username: &str, role: Role) { // self now mutable
        let mut access_control = AccessControl::new();
        access_control.add_role(role);
        self.users.push((username.to_string(), access_control));
    }

    // Check if a user has access to a resource
    pub fn check_access(&mut self, username: &str, resource: &Resource) -> bool { // self now mutable
        if let Some((_, access_control)) = self.users.iter().find(|(user, _)| user == username) {
            if self.rate_limiter.is_within_limit() { // rate_limiter needs mutable borrow, so self must be mutable
                return access_control.has_access(resource);
            }
        }
        false
    }

    // Add a role to an existing user
    pub fn add_role_to_user(&mut self, username: &str, role: Role) {
        if let Some((_, access_control)) = self.users.iter_mut().find(|(user, _)| user == username) {
            access_control.add_role(role);
        }
    }
}
