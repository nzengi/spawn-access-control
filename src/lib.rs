use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// Sahiplik ve Roller yönetimi sağlayan kütüphane
#[wasm_bindgen]
pub struct RoleManager {
    owner: String,
    roles: HashMap<String, Vec<String>>,  // Rol -> Kullanıcı Listesi
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
    pub fn transfer_ownership(&mut self, current_owner: &str, new_owner: &str) -> bool {
        if current_owner != self.owner {
            return false;  // Sadece mevcut sahip transfer yapabilir
        }
        self.owner = new_owner.to_string();
        true
    }

    /// Belirli bir kullanıcıya rol atar (yalnızca sahip yapabilir)
    pub fn assign_role(&mut self, owner: &str, role: &str, user: &str) -> bool {
        if owner != self.owner {
            return false;  // Sadece sahip rol atayabilir
        }

        let role_users = self.roles.entry(role.to_string()).or_insert(Vec::new());
        if !role_users.contains(&user.to_string()) {
            role_users.push(user.to_string());
        }

        true
    }

    /// Bir kullanıcıya atanmış rollerden birini siler (yalnızca sahip yapabilir)
    pub fn remove_role(&mut self, owner: &str, role: &str, user: &str) -> bool {
        if owner != self.owner {
            return false;  // Sadece sahip rol çıkarabilir
        }

        if let Some(role_users) = self.roles.get_mut(role) {
            if let Some(index) = role_users.iter().position(|x| x == user) {
                role_users.remove(index);
            }
        }

        true
    }

    /// Kullanıcının bir role sahip olup olmadığını kontrol eder
    pub fn has_role(&self, role: &str, user: &str) -> bool {
        if let Some(role_users) = self.roles.get(role) {
            return role_users.contains(&user.to_string());
        }
        false
    }

    /// Sahip kontrolü
    pub fn is_owner(&self, user: &str) -> bool {
        self.owner == user
    }

    /// Rol bazlı erişim kontrolü sağlar (sadece sahip veya belirli rolü olanlar çağırabilir)
    pub fn role_based_access(&self, user: &str, role: &str) -> bool {
        self.is_owner(user) || self.has_role(role, user)
    }

    /// Belirli bir rol altında kayıtlı kullanıcıları listeler
    pub fn list_role_users(&self, role: &str) -> Vec<String> {
        self.roles
            .get(role)
            .cloned()
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
        assert!(role_manager.transfer_ownership("owner", "new_owner"));
        assert_eq!(role_manager.get_owner(), "new_owner");

        // Yanlış kullanıcı devretmeye çalışırsa başarısız olur
        assert!(!role_manager.transfer_ownership("wrong_user", "owner"));
    }

    #[test]
    fn test_assign_role() {
        let mut role_manager = RoleManager::new("owner");

        // Rol ataması başarılı
        assert!(role_manager.assign_role("owner", "admin", "user1"));
        assert!(role_manager.has_role("admin", "user1"));

        // Yanlış kullanıcı rol atamaya çalışırsa başarısız olur
        assert!(!role_manager.assign_role("wrong_user", "admin", "user2"));
    }

    #[test]
    fn test_remove_role() {
        let mut role_manager = RoleManager::new("owner");

        // Rol ataması
        role_manager.assign_role("owner", "admin", "user1");
        assert!(role_manager.has_role("admin", "user1"));

        // Rol silme başarılı
        assert!(role_manager.remove_role("owner", "admin", "user1"));
        assert!(!role_manager.has_role("admin", "user1"));

        // Yanlış kullanıcı rol silmeye çalışırsa başarısız olur
        assert!(!role_manager.remove_role("wrong_user", "admin", "user1"));
    }

    #[test]
    fn test_role_based_access() {
        let mut role_manager = RoleManager::new("owner");

        // Sahip erişimi
        assert!(role_manager.role_based_access("owner", "admin"));

        // Rol bazlı erişim
        role_manager.assign_role("owner", "admin", "user1");
        assert!(role_manager.role_based_access("user1", "admin"));

        // Yanlış rol ya da kullanıcı için erişim başarısız
        assert!(!role_manager.role_based_access("user2", "admin"));
    }
}
