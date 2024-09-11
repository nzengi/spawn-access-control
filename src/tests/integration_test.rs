use crate::{AccessManager, Role, Resource, RateLimiter, Permission};

#[test]
fn test_integration_access_control() {
    let mut access_manager = AccessManager::new();
    let admin_role = Role::new("admin", None);
    let resource = Resource::new("secure_file", vec!["admin".to_string()]);

    access_manager.add_user("bob", admin_role.clone());
    assert!(access_manager.check_access("bob", &resource)); // Bob has access as an admin

    // Test rate limiter
    let mut rate_limiter = RateLimiter::new(5, 60); // 5 requests per minute
    for _ in 0..5 {
        assert!(rate_limiter.is_within_limit());
    }
    assert!(!rate_limiter.is_within_limit()); // Rate limit exceeded
}
