# Spawn Access Control Library

A comprehensive and extensible **Access Control Management System** written in **Rust**, supporting WebAssembly for cross-platform compatibility. This library provides advanced features such as role-based access control (RBAC), resource-based permissions, audit logging, session management, rate limiting, and more.

## Features

- **Role-Based Access Control (RBAC):** Assign roles to users and create role hierarchies.
- **Permission Management:** Grant users permissions on specific resources with condition-based access control.
- **Resource-Based Access Control:** Manage resources and define which roles have access.
- **Audit Logging:** Track every access attempt with detailed logs including timestamps and IP addresses.
- **Session Management:** Handle user sessions with expiration and multi-session support.
- **Rate Limiting:** Prevent abuse by limiting the number of requests a user can make within a time window.
- **Multi-Factor Authentication (MFA):** Add an additional layer of security by verifying users through a token system.
- **Caching:** Improve performance by caching user access permissions.
- **WebAssembly Support:** Compile to WebAssembly for cross-platform compatibility.

## Getting Started

To use this library in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
spawn-access-control = "0.1.10"
```
## Example Usage

1. Create a Role-Based Access Control System:

```rust
use spawn_access_control::{Role, AccessManager, Resource};

fn main() {
    let mut access_manager = AccessManager::new();
    
    // Define roles
    let admin_role = Role::new("admin", None);
    let user_role = Role::new("user", None);

    // Define a resource
    let resource = Resource::new("file.txt", vec!["admin".to_string()]);

    // Add a user and assign a role
    access_manager.add_user("alice", admin_role.clone());

    // Check if the user has access to the resource
    if access_manager.check_access("alice", &resource) {
        println!("Access granted!");
    } else {
        println!("Access denied!");
    }
}
```

2. Use Permission-Based Access Control:

```rust
use spawn_access_control::{Permission, AccessManager, Resource};

fn main() {
    let read_permission = Permission { name: "read".to_string(), resource: "file.txt".to_string(), condition: None };
    let write_permission = Permission { name: "write".to_string(), resource: "file.txt".to_string(), condition: None };

    // Use AccessManager to assign roles with permissions
    // Example: Define specific permissions for users based on actions
}
```

3. Rate Limiting:

```rust
use spawn_access_control::RateLimiter;

fn main() {
    let mut rate_limiter = RateLimiter::new(5, 60); // 5 requests per minute

    for _ in 0..5 {
        if rate_limiter.is_within_limit() {
            println!("Request allowed");
        } else {
            println!("Rate limit exceeded");
        }
    }
}
```

## WebAssembly Support

To compile this library to WebAssembly, use the following command:

```bash
wasm-pack build
```

