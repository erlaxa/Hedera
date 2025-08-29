use tauri::command;


#[command]
pub async fn create_account() -> String {
    // Placeholder implementation
    "Account created (placeholder)".to_string()
}

#[command]
pub async fn show_popup() -> String {
    "Hello from Rust!".to_string()
}