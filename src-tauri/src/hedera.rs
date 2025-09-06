use hedera::{AccountId, Client, Hbar, PrivateKey, AccountCreateTransaction};
use serde::Serialize;
use std::str::FromStr;

#[derive(Serialize)]
pub struct CreatedAccount {
  network: String,
  account_id: String,
  private_key: String,
  public_key: String,
  transaction_id: String,
  status: String,
  hashscan_url: String,
}


pub async fn create_account_func(operator_id: String, operator_key: String) -> Result<CreatedAccount, hedera::Error> {
    // Set testnet network for now
    let client = Client::for_testnet();
  
    // Operator creds
    let my_account_id: AccountId = AccountId::from_str(&operator_id).unwrap();
    let my_private_key = PrivateKey::from_str_ecdsa(&operator_key).unwrap();
    // Set operator
    client.set_operator(my_account_id, my_private_key.clone());
  
    // Generate new keys
    let new_private_key = PrivateKey::generate_ecdsa();
    let new_public_key = new_private_key.public_key();
  
    // Build transaction
    let mut tx_create_account = AccountCreateTransaction::new();
    tx_create_account
        .key(new_public_key.clone())
        .alias(new_public_key.to_evm_address().unwrap()) // remove this line if you don’t want alias
        .initial_balance(Hbar::new(1))
        .freeze_with(&client)?; // freeze after setting all parameters
  
    //Sign the transaction with the client operator private key and submit to a Hedera network
    let tx_create_account_response = tx_create_account.execute(&client).await?;
  
    //Request the receipt of the transaction
    let receipt_create_account_tx = tx_create_account_response.get_receipt(&client).await?;
  
    //Get the transaction consensus status
    let status_create_account_tx = receipt_create_account_tx.status;
  
    //Get the Account ID
    let account_id = receipt_create_account_tx.account_id.unwrap();
  
    //Get the Transaction ID 
    let tx_id_account_created = tx_create_account_response.transaction_id.to_string();
  
    // Build your struct
    let result = CreatedAccount {
      network: String::from("testnet"),
      account_id: account_id.to_string(),
      private_key: new_private_key.to_string(),
      public_key: new_public_key.to_string(),
      transaction_id: tx_create_account_response.transaction_id.to_string(),
      status: format!("{:?}", status_create_account_tx),
      hashscan_url: format!(
          "https://hashscan.io/{}/tx/{}",
          "testnet",
          tx_id_account_created
      ),
    };
  
    Ok(result)
}