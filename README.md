# RoleManager

`RoleManager` is a WebAssembly (WASM) library for managing ownership and roles, with secure role-based access control (RBAC). It provides functionality for assigning roles, removing roles, transferring ownership, and enforcing access control policies based on roles or ownership.

## Key Features:
- Ownership transfer and validation
- Role assignment and removal
- Role-based access control (RBAC)

## Example Usage:
```rust
let mut role_manager = RoleManager::new("admin");
role_manager.assign_role("admin", "editor", "user1").unwrap();
assert!(role_manager.has_role("editor", "user1"));
```