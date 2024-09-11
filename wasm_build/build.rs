extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn check_access(role: &str, resource: &str) -> bool {
    let allowed_roles = vec!["admin", "editor"];
    allowed_roles.contains(&role)
}
