use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;
use web_sys::console;

/// # RoleManager: A Library for Managing Ownership and Roles in WASM
///
/// `RoleManager` provides a secure mechanism to manage ownership, roles,
/// and access control in WebAssembly (WASM) environments.
///
/// ## Features:
/// - Ownership transfer
/// - Role assignment and removal
/// - Role-based access control (RBAC)
///
/// ## Usage:
/// RoleManager is designed to manage contract ownership and roles, ensuring only
/// authorized users can perform certain actions.
///
/// ### Example
/// ```rust
/// let mut role_manager = RoleManager::new("admin");
/// role_manager.assign_role("admin", "editor", "user1").unwrap();
/// assert!(role_manager.has_role("editor", "user1"));
/// ```

#[wasm_bindgen]
pub struct RoleManager {
    owner: String,
    roles: HashMap<String, HashSet<String>>,  // Role -> Set of Users
}

#[wasm_bindgen]
impl RoleManager {
    /// Creates a new RoleManager with the specified owner.
    ///
    /// # Arguments
    ///
    /// * `owner` - A string slice that holds the initial owner of the contract.
    ///
    /// # Example
    ///
    /// ```rust
    /// let manager = RoleManager::new("admin");
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(owner: &str) -> RoleManager {
        RoleManager {
            owner: owner.to_string(),
            roles: HashMap::new(),
        }
    }

    /// Returns the current owner of the contract.
    ///
    /// # Example
    ///
    /// ```rust
    /// let owner = role_manager.get_owner();
    /// ```
    pub fn get_owner(&self) -> String {
        self.owner.clone()
    }

    /// Transfers ownership to a new user.
    ///
    /// Only the current owner can transfer ownership.
    ///
    /// # Arguments
    ///
    /// * `current_owner` - The current owner's ID.
    /// * `new_owner` - The new owner's ID.
    ///
    /// # Returns
    ///
    /// * `Ok(())` on success.
    /// * `Err` with a message if the current owner is incorrect.
    pub fn transfer_ownership(&mut self, current_owner: &str, new_owner: &str) -> Result<(), String> {
        if current_owner != self.owner {
            console::log_1(&"Ownership transfer failed: caller is not the owner.".into());
            return Err("Only the current owner can transfer ownership.".into());
        }
        self.owner = new_owner.to_string();
        console::log_1(&format!("Ownership transferred to {}", new_owner).into());
        Ok(())
    }

    /// Assigns a role to a user.
    ///
    /// Only the current owner can assign roles.
    ///
    /// # Arguments
    ///
    /// * `owner` - The current owner's ID.
    /// * `role` - The role to be assigned (e.g., "admin").
    /// * `user` - The user who will receive the role.
    ///
    /// # Returns
    ///
    /// * `Ok(())` on success.
    /// * `Err` if the user already has the role or if the owner is incorrect.
    pub fn assign_role(&mut self, owner: &str, role: &str, user: &str) -> Result<(), String> {
        if owner != self.owner {
            console::log_1(&"Role assignment failed: caller is not the owner.".into());
            return Err("Only the owner can assign roles.".into());
        }

        let role_users = self.roles.entry(role.to_string()).or_insert(HashSet::new());
        if !role_users.contains(user) {
            role_users.insert(user.to_string());
            console::log_1(&format!("Role '{}' assigned to user '{}'", role, user).into());
        } else {
            console::log_1(&"User already has the role.".into());
            return Err("User already has the role.".into());
        }

        Ok(())
    }

    /// Removes a role from a user.
    ///
    /// Only the current owner can remove roles.
    ///
    /// # Arguments
    ///
    /// * `owner` - The current owner's ID.
    /// * `role` - The role to be removed.
    /// * `user` - The user from whom the role will be removed.
    ///
    /// # Returns
    ///
    /// * `Ok(())` on success.
    /// * `Err` if the user doesn't have the role or if the owner is incorrect.
    pub fn remove_role(&mut self, owner: &str, role: &str, user: &str) -> Result<(), String> {
        if owner != self.owner {
            console::log_1(&"Role removal failed: caller is not the owner.".into());
            return Err("Only the owner can remove roles.".into());
        }

        if let Some(role_users) = self.roles.get_mut(role) {
            if role_users.remove(user) {
                console::log_1(&format!("Role '{}' removed from user '{}'", role, user).into());
                if role_users.is_empty() {
                    self.roles.remove(role); // Remove the role if no users left
                    console::log_1(&format!("Role '{}' is removed as no users left.", role).into());
                }
                return Ok(());
            }
        }

        console::log_1(&"Role or user not found.".into());
        Err("Role or user not found.".into())
    }

    /// Checks if a user has a specific role.
    ///
    /// # Arguments
    ///
    /// * `role` - The role to check (e.g., "admin").
    /// * `user` - The user to check.
    ///
    /// # Returns
    ///
    /// * `true` if the user has the role.
    /// * `false` if the user does not have the role.
    pub fn has_role(&self, role: &str, user: &str) -> bool {
        if let Some(role_users) = self.roles.get(role) {
            return role_users.contains(user);
        }
        false
    }

    /// Verifies if a user is the owner.
    ///
    /// # Arguments
    ///
    /// * `user` - The user to check.
    ///
    /// # Returns
    ///
    /// * `Ok(true)` if the user is the owner.
    /// * `Err` with a message if the user is not the owner.
    pub fn is_owner(&self, user: &str) -> Result<bool, String> {
        if self.owner == user {
            Ok(true)
        } else {
            console::log_1(&"Caller is not the owner.".into());
            Err("Caller is not the owner.".into())
        }
    }

    /// Provides role-based access control.
    ///
    /// # Arguments
    ///
    /// * `user` - The user requesting access.
    /// * `role` - The required role for access.
    ///
    /// # Returns
    ///
    /// * `true` if the user is the owner or has the role.
    /// * `false` otherwise.
    pub fn role_based_access(&self, user: &str, role: &str) -> bool {
        match self.is_owner(user) {
            Ok(true) => true,
            _ => self.has_role(role, user),
        }
    }

    /// Lists all users for a given role.
    ///
    /// # Arguments
    ///
    /// * `role` - The role to list users for.
    ///
    /// # Returns
    ///
    /// * A `Vec<String>` containing all users with the specified role.
    pub fn list_role_users(&self, role: &str) -> Vec<String> {
        self.roles
            .get(role)
            .map(|users| users.iter().cloned().collect())
            .unwrap_or_else(Vec::new)
    }
}