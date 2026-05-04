#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Address, Vec, Symbol, token};

#[contract]
pub struct FareSplitContract;

#[contractimpl]
impl FareSplitContract {

    // Create a ride pool with driver and total fare
    pub fn create_pool(env: Env, pool_id: Symbol, driver: Address, fare: i128) {
        env.storage().instance().set(&(pool_id.clone(), Symbol::short("driver")), &driver);
        env.storage().instance().set(&(pool_id.clone(), Symbol::short("fare")), &fare);
        env.storage().instance().set(&(pool_id.clone(), Symbol::short("settled")), &false);

        let members: Vec<Address> = Vec::new(&env);
        env.storage().instance().set(&(pool_id, Symbol::short("members")), &members);
    }

    // Join a ride pool
    pub fn join_pool(env: Env, pool_id: Symbol, member: Address) {
        let mut members: Vec<Address> = env.storage()
            .instance()
            .get(&(pool_id.clone(), Symbol::short("members")))
            .unwrap();

        members.push_back(member);
        env.storage().instance().set(&(pool_id, Symbol::short("members")), &members);
    }

    // Settle payment using USDC token contract
    pub fn settle_pool(env: Env, pool_id: Symbol, token: Address, payer: Address) {

        let settled: bool = env.storage()
            .instance()
            .get(&(pool_id.clone(), Symbol::short("settled")))
            .unwrap();

        if settled {
            panic!("Pool already settled");
        }

        let driver: Address = env.storage()
            .instance()
            .get(&(pool_id.clone(), Symbol::short("driver")))
            .unwrap();

        let fare: i128 = env.storage()
            .instance()
            .get(&(pool_id.clone(), Symbol::short("fare")))
            .unwrap();

        let members: Vec<Address> = env.storage()
            .instance()
            .get(&(pool_id.clone(), Symbol::short("members")))
            .unwrap();

        let total_people = (members.len() as i128) + 1;
        let share = fare / total_people;

        let token_client = token::Client::new(&env, &token);

        // transfer to driver
        token_client.transfer(&payer, &driver, &fare);

        // mark settled
        env.storage().instance().set(&(pool_id, Symbol::short("settled")), &true);
    }

    // View pool status
    pub fn is_settled(env: Env, pool_id: Symbol) -> bool {
        env.storage()
            .instance()
            .get(&(pool_id, Symbol::short("settled")))
            .unwrap_or(false)
    }
}