use crate::{AccessManager, Role, Resource, Permission, RateLimiter};

#[test]
fn test_role_hierarchy() {
    let admin = Role::new("admin", None);
    let super_admin = Role::new("super_admin", Some(Box::new(admin.clone())));

    assert!(super_admin.is_higher_or_equal(&admin));
}

#[test]
fn test_permission_granting() {
    let read_permission = Permission { name: "read".to_string(), resource: "file".to_string(), condition: None };
    let write_permission = Permission { name: "write".to_string(), resource: "file".to_string(), condition: None };
    let permissions = vec![read_permission, write_permission];

    assert!(permissions.iter().any(|p| p.name == "read"));
}

#[test]
fn test_rate_limiter() {
    let mut rate_limiter = RateLimiter::new(10, 60); // 10 requests per minute

    for _ in 0..10 {
        assert!(rate_limiter.is_within_limit());
    }

    assert!(!rate_limiter.is_within_limit()); // Exceeded the limit
}
