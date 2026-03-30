#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct AllowanceTracker;

const BALANCE: Symbol = symbol_short!("BALANCE");

#[contractimpl]
impl AllowanceTracker {
    // Sets the initial stipend amount
    pub fn init(env: Env, total: u32) {
        env.storage().instance().set(&BALANCE, &total);
    }

    // Deducts the claimed amount from the total if there are enough funds
    pub fn claim(env: Env, amount: u32) {
        let mut current: u32 = env.storage().instance().get(&BALANCE).unwrap_or(0);
        if current >= amount {
            current -= amount;
            env.storage().instance().set(&BALANCE, &current);
        } else {
            panic!("Insufficient funds");
        }
    }

    // Returns the current balance
    pub fn get_bal(env: Env) -> u32 {
        env.storage().instance().get(&BALANCE).unwrap_or(0)
    }
}

mod test;