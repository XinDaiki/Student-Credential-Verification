#![no_std]

use soroban_sdk::{
    contract, contractimpl, symbol_short, Env, Symbol, String, Map, BytesN
};

#[contract]
pub struct StudentCred;

const STUDENTS: Symbol = symbol_short!("STUDENTS");

#[contractimpl]
impl StudentCred {
    //  Add student
    pub fn add_student(env: Env, name: String, course: String) -> BytesN<32> {
        let mut students: Map<BytesN<32>, String> =
            env.storage().instance().get(&STUDENTS).unwrap_or(Map::new(&env));

        let hash = Self::generate_hash(&env, &name, &course);

        students.set(hash.clone(), name.clone());
        env.storage().instance().set(&STUDENTS, &students);

        hash
    }

    // Verify
    pub fn verify(env: Env, hash: BytesN<32>) -> bool {
        let students: Map<BytesN<32>, String> =
            env.storage().instance().get(&STUDENTS).unwrap_or(Map::new(&env));

        students.contains_key(hash)
    }

    // HASH FUNCTION (FIXED)
    fn generate_hash(env: &Env, name: &String, course: &String) -> BytesN<32> {
    let mut bytes = name.to_bytes();

    let dash = String::from_str(env, "-").to_bytes();
    let course_bytes = course.to_bytes();

    bytes.append(&dash);
    bytes.append(&course_bytes);

    env.crypto().sha256(&bytes).into()
}
}
