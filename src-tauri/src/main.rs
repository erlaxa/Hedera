// src-tauri/src/main.rs
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use hedera::{AccountCreateTransaction, AccountId, Client, Hbar, PrivateKey};
use serde::Serialize;
use std::str::FromStr;

#[derive(Serialize)]
struct CreatedAccount {
  network: String,
  account_id: String,
  private_key: String,
  public_key: String,
  transaction_id: String,
  status: String,
  hashscan_url: String,
}

#[tauri::command]
async fn create_account(use_testnet: bool, operator_id: String, operator_key: String) -> Result<CreatedAccount, String> {
  // Choose network
  let mut client = if use_testnet {
      Client::for_testnet()
  } else {
      Client::for_mainnet()
  };

  // Operator creds
  let my_account_id = AccountId::from_str(&operator_id)
      .map_err(|e| format!("Invalid account ID: {:?}", e))?;
  let my_private_key = PrivateKey::from_str_ed25519(&operator_key)
      .map_err(|e| format!("Invalid private key: {:?}", e))?;
  client.set_operator(my_account_id, my_private_key);

  // Generate new keys
  let new_private_key = PrivateKey::generate_ecdsa();
  let new_public_key = new_private_key.public_key();

  // Transaction
  let tx = AccountCreateTransaction::new()
      .key(new_public_key.clone())
      .alias(new_public_key.to_evm_address().unwrap())
      .initial_balance(Hbar::new(10))
      .freeze_with(&client)
      .map_err(|e| format!("Freeze error: {:?}", e))?;

  let response = tx.execute(&client)
      .await
      .map_err(|e| format!("Execute error: {:?}", e))?;

  let receipt = response.get_receipt(&client)
      .await
      .map_err(|e| format!("Receipt error: {:?}", e))?;

  let account_id = receipt.account_id
      .ok_or("No account ID in receipt".to_string())?;

  // Return as JSON to Angular
  Ok(CreatedAccount {
      network: if use_testnet { "testnet".into() } else { "mainnet".into() },
      account_id: account_id.to_string(),
      private_key: new_private_key.to_string(),
      public_key: new_public_key.to_string(),
      transaction_id: response.transaction_id.to_string(),
      status: receipt.status.to_string(),
      hashscan_url: format!(
          "https://hashscan.io/{}/tx/{}",
          if use_testnet { "testnet" } else { "mainnet" },
          response.transaction_id
      ),
  })
}

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![create_account])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
