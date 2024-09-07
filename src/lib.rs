use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;
use web_sys::console;

/// Sahiplik ve Roller yönetimi sağlayan kütüphane
#[wasm_bindgen]
pub struct RoleManager {
    owner: String,
    roles: HashMap<String, HashSet<String>>,  // Rol -> Kullanıcı Seti
}

#[wasm_bindgen]
impl RoleManager {
    /// Yeni bir kontrat sahipliği ve roller yönetimi başlatır
    #[wasm_bindgen(constructor)]
    pub fn new(owner: &str) -> RoleManager {
        RoleManager {
            owner: owner.to_string(),
            roles: HashMap::new(),
        }
    }

    /// Sahibi döndürür
    pub fn get_owner(&self) -> String {
        self.owner.clone()
    }

    /// Sahipliği başka bir kullanıcıya devreder (yalnızca mevcut sahip çağırabilir)
    pub fn transfer_ownership(&mut self, current_owner: &str, new_owner: &str) -> Result<(), String> {
        if current_owner != self.owner {
            console::log_1(&"Ownership transfer failed: caller is not the owner.".into());
            return Err("Only the current owner can transfer ownership.".into());
        }
        self.owner = new_owner.to_string();
        console::log_1(&format!("Ownership transferred to {}", new_owner).into());
        Ok(())
    }

    /// Belirli bir kullanıcıya rol atar (yalnızca sahip yapabilir)
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

    /// Bir kullanıcıya atanmış rollerden birini siler (yalnızca sahip yapabilir)
    pub fn remove_role(&mut self, owner: &str, role: &str, user: &str) -> Result<(), String> {
        if owner != self.owner {
            console::log_1(&"Role removal failed: caller is not the owner.".into());
            return Err("Only the owner can remove roles.".into());
        }

        if let Some(role_users) = self.roles.get_mut(role) {
            if role_users.remove(user) {
                console::log_1(&format!("Role '{}' removed from user '{}'", role, user).into());
                if role_users.is_empty() {
                    self.roles.remove(role); // Kullanıcı listesi boşsa rolü kaldır
                    console::log_1(&format!("Role '{}' is removed as no users left.", role).into());
                }
                return Ok(());
            }
        }

        console::log_1(&"Role or user not found.".into());
        Err("Role or user not found.".into())
    }

    /// Kullanıcının bir role sahip olup olmadığını kontrol eder
    pub fn has_role(&self, role: &str, user: &str) -> bool {
        if let Some(role_users) = self.roles.get(role) {
            return role_users.contains(user);
        }
        false
    }

    /// Sahip kontrolü (Hata mesajıyla birlikte döndürür)
    pub fn is_owner(&self, user: &str) -> Result<bool, String> {
        if self.owner == user {
            Ok(true)
        } else {
            console::log_1(&"Caller is not the owner.".into());
            Err("Caller is not the owner.".into())
        }
    }

    /// Rol bazlı erişim kontrolü sağlar (sadece sahip veya belirli rolü olanlar çağırabilir)
    pub fn role_based_access(&self, user: &str, role: &str) -> bool {
        match self.is_owner(user) {
            Ok(true) => true,
            _ => self.has_role(role, user),
        }
    }

    /// Belirli bir rol altında kayıtlı kullanıcıları listeler
    pub fn list_role_users(&self, role: &str) -> Vec<String> {
        self.roles
            .get(role)
            .map(|users| users.iter().cloned().collect())
            .unwrap_or_else(Vec::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_transfer() {
        let mut role_manager = RoleManager::new("owner");

        // Doğru sahip devretme işlemi
        assert!(role_manager.transfer_ownership("owner", "new_owner").is_ok());
        assert_eq!(role_manager.get_owner(), "new_owner");

        // Yanlış kullanıcı devretmeye çalışırsa başarısız olur
        assert!(role_manager.transfer_ownership("wrong_user", "owner").is_err());
    }

    #[test]
    fn test_assign_role() {
        let mut role_manager = RoleManager::new("owner");

        // Rol ataması başarılı
        assert!(role_manager.assign_role("owner", "admin", "user1").is_ok());
        assert!(role_manager.has_role("admin", "user1"));

        // Aynı kullanıcıya tekrar rol ataması başarısız olmalı
        assert!(role_manager.assign_role("owner", "admin", "user1").is_err());

        // Yanlış kullanıcı rol atamaya çalışırsa başarısız olur
        assert!(role_manager.assign_role("wrong_user", "admin", "user2").is_err());
    }

    #[test]
    fn test_remove_role() {
        let mut role_manager = RoleManager::new("owner");

        // Rol ataması
        role_manager.assign_role("owner", "admin", "user1").unwrap();
        assert!(role_manager.has_role("admin", "user1"));

        // Rol silme başarılı
        assert!(role_manager.remove_role("owner", "admin", "user1").is_ok());
        assert!(!role_manager.has_role("admin", "user1"));

        // Yanlış kullanıcı rol silmeye çalışırsa başarısız olur
        assert!(role_manager.remove_role("wrong_user", "admin", "user1").is_err());
    }

    #[test]
    fn test_role_based_access() {
        let mut role_manager = RoleManager::new("owner");

        // Sahip erişimi
        assert!(role_manager.role_based_access("owner", "admin"));

        // Rol bazlı erişim
        role_manager.assign_role("owner", "admin", "user1").unwrap();
        assert!(role_manager.role_based_access("user1", "admin"));

        // Yanlış rol ya da kullanıcı için erişim başarısız
        assert!(!role_manager.role_based_access("user2", "admin"));
    }
}
