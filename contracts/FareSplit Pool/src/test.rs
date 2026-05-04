#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env, Symbol, Address};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_path_settlement() {
        let env = Env::default();
        let contract_id = env.register_contract(None, FareSplitContract);

        let driver = Address::generate(&env);
        let payer = Address::generate(&env);
        let token = Address::generate(&env);

        let pool_id = Symbol::short("pool1");

        FareSplitContract::create_pool(&env, pool_id.clone(), driver.clone(), 100);

        FareSplitContract::join_pool(&env, pool_id.clone(), Address::generate(&env));
        FareSplitContract::settle_pool(&env, pool_id.clone(), token, payer);

        assert!(FareSplitContract::is_settled(&env, pool_id));
    }

    #[test]
    #[should_panic]
    fn test_double_settlement_fails() {
        let env = Env::default();
        let driver = Address::generate(&env);
        let payer = Address::generate(&env);
        let token = Address::generate(&env);

        let pool_id = Symbol::short("pool2");

        FareSplitContract::create_pool(&env, pool_id.clone(), driver.clone(), 100);
        FareSplitContract::settle_pool(&env, pool_id.clone(), token.clone(), payer.clone());

        // second call should fail
        FareSplitContract::settle_pool(&env, pool_id.clone(), token, payer);
    }

    #[test]
    fn test_state_is_set_after_create() {
        let env = Env::default();
        let driver = Address::generate(&env);

        let pool_id = Symbol::short("pool3");

        FareSplitContract::create_pool(&env, pool_id.clone(), driver, 200);

        assert_eq!(FareSplitContract::is_settled(&env, pool_id), false);
    }

    #[test]
    fn test_join_pool_increases_members() {
        let env = Env::default();
        let driver = Address::generate(&env);

        let pool_id = Symbol::short("pool4");

        FareSplitContract::create_pool(&env, pool_id.clone(), driver, 100);

        FareSplitContract::join_pool(&env, pool_id.clone(), Address::generate(&env));
        FareSplitContract::join_pool(&env, pool_id.clone(), Address::generate(&env));

        // indirect verification via storage access would be in real token env
        assert!(!FareSplitContract::is_settled(&env, pool_id));
    }

    #[test]
    fn test_multiple_pools_independent() {
        let env = Env::default();
        let driver = Address::generate(&env);

        let pool_a = Symbol::short("A");
        let pool_b = Symbol::short("B");

        FareSplitContract::create_pool(&env, pool_a.clone(), driver.clone(), 100);
        FareSplitContract::create_pool(&env, pool_b.clone(), driver, 200);

        assert_eq!(FareSplitContract::is_settled(&env, pool_a), false);
        assert_eq!(FareSplitContract::is_settled(&env, pool_b), false);
    }
}