// src-tauri/src/main.rs
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crate::hedera::CreatedAccount;

mod hedera;

#[tauri::command]
async fn create_account(operator_id: String, operator_key: String) -> Result<CreatedAccount, String> {
    match crate::hedera::create_account_func(operator_id, operator_key).await {
        Ok(account) => Ok(account),
        Err(_) => Err("Failed to create account. Please check your operator credentials or network connection.".into()),
    }
}

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![create_account])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
