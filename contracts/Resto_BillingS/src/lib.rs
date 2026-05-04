#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Address, Map};

#[contract]
pub struct SplitBillContract;

// Storage keys
fn bill_key(env: &Env, bill_id: u32) -> Symbol {
    Symbol::new(env, "BILL")
}

#[contractimpl]
impl SplitBillContract {

    // Create a new bill with total and participants
    pub fn create_bill(env: Env, bill_id: u32, payer: Address, total: i128, participants: Map<Address, i128>) {
        payer.require_auth();

        env.storage().instance().set(&bill_id, &(payer, total, participants));
    }

    // Pay share into contract
    pub fn pay_share(env: Env, bill_id: u32, from: Address, amount: i128) {
        from.require_auth();

        let (payer, total, mut participants): (Address, i128, Map<Address, i128>) =
            env.storage().instance().get(&bill_id).unwrap();

        let owed = participants.get(from.clone()).unwrap();
        assert!(amount == owed, "Incorrect amount");

        participants.set(from.clone(), 0);

        env.storage().instance().set(&bill_id, &(payer, total, participants));
    }

    // Release funds if all paid
    pub fn settle(env: Env, bill_id: u32) {
        let (payer, total, participants): (Address, i128, Map<Address, i128>) =
            env.storage().instance().get(&bill_id).unwrap();

        for (_, amount) in participants.iter() {
            assert!(amount == 0, "Not fully paid");
        }

        // In real implementation: transfer USDC to payer
        env.storage().instance().remove(&bill_id);
    }
}