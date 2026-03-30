#![cfg(test)]
use super::*;
use soroban_sdk::Env;

#[test]
fn test_init() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AllowanceTracker);
    let client = AllowanceTrackerClient::new(&env, &contract_id);
    
    client.init(&1000);
    assert_eq!(client.get_bal(), 1000);
}

#[test]
fn test_claim_success() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AllowanceTracker);
    let client = AllowanceTrackerClient::new(&env, &contract_id);
    
    client.init(&1000);
    client.claim(&200);
    assert_eq!(client.get_bal(), 800); // 1000 - 200 = 800
}

#[test]
#[should_panic(expected = "Insufficient funds")]
fn test_claim_fail() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AllowanceTracker);
    let client = AllowanceTrackerClient::new(&env, &contract_id);
    
    client.init(&100);
    client.claim(&200); // Trying to claim 200 when only 100 is available
}