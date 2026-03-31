#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String, BytesN};

#[test]
fn test_add_and_verify() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StudentCred);
    let client = StudentCredClient::new(&env, &contract_id);

    let name = String::from_str(&env, "Juan Dela Cruz");
    let course = String::from_str(&env, "BSIT");

    // Get the REAL hash returned from contract
    let hash: BytesN<32> = client.add_student(&name, &course);

    // Use the correct hash
    assert!(client.verify(&hash));
}

#[test]
fn test_fake_student() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StudentCred);
    let client = StudentCredClient::new(&env, &contract_id);

    // Create a fake hash (all zeros)
    let fake_hash = BytesN::from_array(&env, &[0; 32]);

    assert!(!client.verify(&fake_hash));
}
