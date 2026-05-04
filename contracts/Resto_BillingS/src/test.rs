#[cfg(test)]
mod tests {
    use soroban_sdk::{Env, Address, Map};
    use crate::SplitBillContract;

    #[test]
    fn test_happy_path() {
        let env = Env::default();
        let contract = SplitBillContract;

        let payer = Address::random(&env);
        let user1 = Address::random(&env);

        let mut participants = Map::new(&env);
        participants.set(user1.clone(), 100);

        contract.create_bill(env.clone(), 1, payer.clone(), 100, participants.clone());
        contract.pay_share(env.clone(), 1, user1.clone(), 100);
        contract.settle(env.clone(), 1);
    }

    #[test]
    fn test_wrong_amount() {
        let env = Env::default();
        let contract = SplitBillContract;

        let payer = Address::random(&env);
        let user1 = Address::random(&env);

        let mut participants = Map::new(&env);
        participants.set(user1.clone(), 100);

        contract.create_bill(env.clone(), 1, payer, 100, participants.clone());

        let result = std::panic::catch_unwind(|| {
            contract.pay_share(env.clone(), 1, user1.clone(), 50);
        });

        assert!(result.is_err());
    }

    #[test]
    fn test_state_update() {
        let env = Env::default();
        let contract = SplitBillContract;

        let payer = Address::random(&env);
        let user1 = Address::random(&env);

        let mut participants = Map::new(&env);
        participants.set(user1.clone(), 100);

        contract.create_bill(env.clone(), 1, payer.clone(), 100, participants.clone());
        contract.pay_share(env.clone(), 1, user1.clone(), 100);

        let (_, _, updated): (Address, i128, Map<Address, i128>) =
            env.storage().instance().get(&1).unwrap();

        assert_eq!(updated.get(user1).unwrap(), 0);
    }

    #[test]
    fn test_settle_fail_if_unpaid() {
        let env = Env::default();
        let contract = SplitBillContract;

        let payer = Address::random(&env);
        let user1 = Address::random(&env);

        let mut participants = Map::new(&env);
        participants.set(user1.clone(), 100);

        contract.create_bill(env.clone(), 1, payer, 100, participants);

        let result = std::panic::catch_unwind(|| {
            contract.settle(env.clone(), 1);
        });

        assert!(result.is_err());
    }

    #[test]
    fn test_bill_removed_after_settle() {
        let env = Env::default();
        let contract = SplitBillContract;

        let payer = Address::random(&env);
        let user1 = Address::random(&env);

        let mut participants = Map::new(&env);
        participants.set(user1.clone(), 100);

        contract.create_bill(env.clone(), 1, payer.clone(), 100, participants.clone());
        contract.pay_share(env.clone(), 1, user1.clone(), 100);
        contract.settle(env.clone(), 1);

        let exists = env.storage().instance().has(&1);
        assert!(!exists);
    }
}