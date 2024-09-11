// src/audit_log.rs
#[derive(Debug, Clone)]
pub struct AuditLog {
    pub user: String,
    pub resource: String,
    pub action: String,  // read, write, delete
    pub access_granted: bool,
    pub timestamp: u64,
    pub ip_address: String,  // User's IP address
}

impl AuditLog {
    pub fn new(user: &str, resource: &str, action: &str, access_granted: bool, ip_address: &str) -> Self {
        AuditLog {
            user: user.to_string(),
            resource: resource.to_string(),
            action: action.to_string(),
            access_granted,
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            ip_address: ip_address.to_string(),
        }
    }
}
