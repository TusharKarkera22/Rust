use std::collections::HashMap;
use std::sync::Mutex;
use std::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Account {
    address: String,
    balance: u64,
}

#[derive(Debug)]
struct Wallet {
    accounts: Mutex<HashMap<String, Account>>,
}

impl Wallet {
    fn new() -> Self {
        let mut accounts = HashMap::new();
        // Create default account
        accounts.insert(
            "default".to_string(),
            Account {
                address: "default".to_string(),
                balance: 1000,
            },
        );
        
        Wallet {
            accounts: Mutex::new(accounts),
        }
    }

    fn get_balance(&self, address: &str) -> u64 {
        let accounts = self.accounts.lock().unwrap();
        accounts.get(address)
            .map(|account| account.balance)
            .unwrap_or(0)
    }

    fn transfer(&self, from: &str, to: &str, amount: u64) -> Result<(), String> {
        let mut accounts = self.accounts.lock().unwrap();
        
        // Check if sender has enough balance
        if let Some(sender) = accounts.get_mut(from) {
            if sender.balance < amount {
                return Err("Insufficient balance".to_string());
            }
            sender.balance -= amount;
        } else {
            return Err("Sender account not found".to_string());
        }
        
        // Update or create receiver account
        accounts
            .entry(to.to_string())
            .and_modify(|account| account.balance += amount)
            .or_insert(Account {
                address: to.to_string(),
                balance: amount,
            });
            
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let wallet = Wallet::new();
    
    // Create some test accounts
    println!("Initial balance of default account: {}", wallet.get_balance("default"));
    
    // Perform a transfer
    match wallet.transfer("default", "alice", 500) {
        Ok(_) => println!("Transfer successful!"),
        Err(e) => println!("Transfer failed: {}", e),
    }
    
    // Check balances after transfer
    println!("Default account balance: {}", wallet.get_balance("default"));
    println!("Alice's balance: {}", wallet.get_balance("alice"));
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer() {
        let wallet = Wallet::new();
        assert!(wallet.transfer("default", "bob", 300).is_ok());
        assert_eq!(wallet.get_balance("bob"), 300);
        assert_eq!(wallet.get_balance("default"), 700);
    }

    #[test]
    fn test_insufficient_balance() {
        let wallet = Wallet::new();
        assert!(wallet.transfer("default", "charlie", 2000).is_err());
    }
}