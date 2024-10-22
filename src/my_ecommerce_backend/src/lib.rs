use std::collections::HashMap;
use ic_cdk::export::Principal;
use ic_cdk_macros::{update, query};

#[derive(Default)]
struct PaymentCanister {
    balances: HashMap<Principal, u64>, // Store user balances
}

impl PaymentCanister {
    #[update]
    fn process_payment(&mut self, payer: Principal, amount: u64) -> Result<(), String> {
        let user_balance = self.balances.entry(payer).or_insert(0);
        if *user_balance < amount {
            return Err("Insufficient balance".to_string());
        }
        *user_balance -= amount;
        Ok(())
    }

    #[query]
    fn get_balance(&self, user: Principal) -> u64 {
        *self.balances.get(&user).unwrap_or(&0)
    }

    #[update]
    fn add_funds(&mut self, user: Principal, amount: u64) {
        let balance = self.balances.entry(user).or_insert(0);
        *balance += amount;
    }
}

