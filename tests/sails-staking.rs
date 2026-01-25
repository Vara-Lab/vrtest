use vrtest::{
    contract::{builders::UploadWasmT, Contract}, runtime::*, types::{
        builtin_staking::RewardAccount,
        actorid32::ActorId32,
        StakingEventType, 
        CONTRACT_EXISTENCIAL_DEPOSIT, 
        ENDOWMENT, 
        ONE_TOKEN, 
        SESSION_DURATION_IN_BLOCKS
    } 
};
use common::Origin;
use demo_sails_staking_broker::WASM_BINARY;

const REWARD_PAYEE: u64 = 15;
const SIGNER: u64 = 1;
const VAL_1_STASH: u64 = 10;
const VAL_1_STASH_AUTH_ID: u64 = 11;
const VAL_2_STASH: u64 = 20;
const VAL_2_STASH_AUTH_ID: u64 = 21;
const VAL_3_STASH: u64 = 30;
const VAL_3_STASH_AUTH_ID: u64 = 31;
pub const DEFAULT_GAS_LIMIT: u64 = 20_000_000_000;

#[test]
fn bonding_works() {
    init_logger();

    new_test_ext(vec![SIGNER]).execute_with(|| {
        // This pours the ED onto the contract's account (1 token as testnet)
        let contract = Contract::upload_sails_contract::<()>()
            .signer(SIGNER)
            .salt("contract")
            .app_constructor_name("New")
            .wasm(WASM_BINARY)
            .upload();

        let signer_current_balance_at_block_1 = balance_from_user(SIGNER);

        // Measure necessary gas in a transaction
        let gas_fees = contract.new_calculate_gas()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Bond")
            .add_arg((
                100 * ONE_TOKEN,
                RewardAccount::Program
            ))
            .with_value(100 * ONE_TOKEN)
            .calculate_gas();

        // Ensure the state hasn't changed
        assert_eq!(
            signer_current_balance_at_block_1,
            balance_from_user(SIGNER)
        );

        // Asserting success

        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Bond")
            .add_arg(100 * ONE_TOKEN)
            .add_arg(RewardAccount::Program)
            .with_value(100 * ONE_TOKEN)
            .send_and_run_one_block();

        assert!(result.is_ok());

        let signer_current_balance_at_block_2 = balance_from_user(SIGNER);
        
        // SIGNER has spent in current block:
        // - 100 UNITS sent as value to the contract
        // - paid for the burned gas

        assert_eq!(
            signer_current_balance_at_block_2,
            signer_current_balance_at_block_1 - 100 * ONE_TOKEN - gas_fees.burned // gas_price(gas_fees)
        );

        // The contract's account has 1 * ONE_TOKEN of the ED and 100 * ONE_TOKEN of the bonded funds
        assert_eq!(contract.free_balance(), 100 * ONE_TOKEN + CONTRACT_EXISTENCIAL_DEPOSIT);
        // and all of it is frozen as bonded or locked
        assert_eq!(contract.frozen_balance(), 100 * ONE_TOKEN);
        
        // Asserting the expected events are present
        assert_staking_events(&contract, 100 * ONE_TOKEN, StakingEventType::Bonded);

        reset_system_events();

        // Measure necessary gas again as underlying runtime call should be different this time:
        // - `bond_extra` instead of `bond`
        let gas_fees = contract.new_calculate_gas()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Bond")
            .add_arg((
                50 * ONE_TOKEN,
                RewardAccount::Program
            ))
            .with_value(100 * ONE_TOKEN)
            .calculate_gas();

        // Asserting success again (the contract should be able to figure out that `bond_extra`
        // should be called instead).
        // Note: the actual added amount is limited by the message `value` field, that is
        // it's going to be 50 UNITS, not 100 UNITS as encoded in the message payload.

        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Bond")
            .add_arg(50 * ONE_TOKEN)
            .add_arg(RewardAccount::Program)
            .with_value(100 * ONE_TOKEN)
            .send_and_run_one_block();

        assert!(result.is_ok());

        // SIGNER has spent since last time:
        // - 50 UNITS sent as value to the contract
        // - paid for gas
        assert_eq!(
            balance_from_user(SIGNER),
            signer_current_balance_at_block_2 - 100 * ONE_TOKEN - gas_fees.burned // gas_price(gas_fees)
        );

        // Another 50 * UNITS added to locked balance
        assert_eq!(
            contract.frozen_balance(),
            150 * ONE_TOKEN
        );

        // Asserting the expected events are present
        assert_staking_events(&contract, 50 * ONE_TOKEN, StakingEventType::Bonded);

    });
}

#[test]
fn unbonding_works() {
    init_logger();    

    new_test_ext(vec![SIGNER])
    .execute_with(|| {
        let contract = Contract::upload_sails_contract::<()>()
            .signer(SIGNER)
            .salt("contract")
            .wasm(WASM_BINARY)
            .app_constructor_name("New")
            .upload();

        let _result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Bond")
            .add_arg(100 * ONE_TOKEN)
            .add_arg(RewardAccount::Program)
            .with_value(100 * ONE_TOKEN)
            .send_and_run_one_block();

        run_to_next_block();

        // Asserting the expected events are present
        assert_staking_events(&contract, 100 * ONE_TOKEN, StakingEventType::Bonded);

        reset_system_events();

        // Measure necessary gas in a transaction for `unbond` message
        let _gas_fees = contract.new_calculate_gas()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Unbond")
            .add_arg(200 * ONE_TOKEN)
            .calculate_gas();

        // Sending `unbond` message
        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Unbond")
            .add_arg(200 * ONE_TOKEN)
            .send_and_run_one_block();

        assert!(result.is_ok());

        // Asserting the expected events are present
        assert_staking_events(&contract, 100 * ONE_TOKEN, StakingEventType::Unbonded);
    });
}

#[test]
fn nominators_payload_size_matters() {
    init_logger();

    new_test_ext(vec![SIGNER]).execute_with(|| {
        let contract = Contract::upload_sails_contract::<()>()
            .signer(SIGNER)
            .salt("contract")
            .app_constructor_name("New")
            .wasm(WASM_BINARY)
            .upload();

        // Prepare large payload
        let mut targets = Vec::<ActorId32>::new();
        for i in 100_u64..200_u64 {
            // targets.push(i.cast());
            targets.push(ActorId32::from(i));
        }

        reset_system_events();

        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Nominate")
            .add_arg(targets)
            .send_and_run_one_block();

        assert!(result.is_ok());

        // No staking-related events should have been emitted
        assert_no_staking_events();

        // Error message has been sent to the user
        assert_error_message_sent();

        // User message payload indicates the error
        assert_payload_contains("Message decoding error");
    });
}

#[test]
fn nominating_works() {
    init_logger();

    let authorities = vec![
        VAL_1_STASH,
        VAL_2_STASH
    ];

    new_test_ext_with_authorities(
        vec![SIGNER],
        authorities
    ).execute_with(|| {
        let contract = Contract::upload_sails_contract::<()>()
            .signer(SIGNER)
            .salt("contract")
            .app_constructor_name("New")
            .wasm(WASM_BINARY)
            .upload();

        // let targets: Vec<ActorId> = vec![VAL_1_STASH, VAL_2_STASH]
        let targets: Vec<ActorId32> = vec![VAL_1_STASH, VAL_2_STASH]
            .into_iter()
            // .map(|x| x.cast())
            .map(|x| ActorId32::from(x))
            .collect();

        // Doesn't work without bonding first
        reset_system_events();

        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Nominate")
            .add_arg(targets.clone())
            .send_and_run_one_block();

        assert!(result.is_ok());

        // No staking-related events should have been emitted
        assert_no_staking_events();
        // Error message has been sent to the user
        assert_error_message_sent();

        // Bond some funds on behalf of the contract first
        reset_system_events();

        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Bond")
            .add_arg(100 * ONE_TOKEN)
            .add_arg(RewardAccount::Program)
            .with_value(100 * ONE_TOKEN)
            .send_and_run_one_block();

        assert!(result.is_ok());

        assert_staking_events(&contract, 100 * ONE_TOKEN, StakingEventType::Bonded);

        let targets_before = contract_nominators(&contract);
        
        assert_eq!(targets_before.len(), 0);

        // Now expecting nominating to work

        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Nominate")
            .add_arg(targets.clone())
            .send_and_run_one_block();

        assert!(result.is_ok());

        run_to_next_block();

        let contract_nominators = contract.nominators(); 

        assert_eq!(contract_nominators.len(), targets.len());
    });
}

#[test]
fn withdraw_unbonded_works() {
    init_logger();

    let authorities = vec![
        VAL_1_STASH,
        VAL_2_STASH
    ];

    new_test_ext_with_authorities(
        vec![SIGNER],
        authorities
    ).execute_with(|| {
        let contract = Contract::upload_sails_contract::<()>()
            .signer(SIGNER)
            .salt("contract")
            .app_constructor_name("New")
            .wasm(WASM_BINARY)
            .upload();

        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Bond")
            .add_arg(500 * ONE_TOKEN)
            .add_arg(RewardAccount::Program)
            .with_value(500 * ONE_TOKEN)
            .send_and_run_one_block();

        assert!(result.is_ok());

        assert_staking_events(&contract, 500 * ONE_TOKEN, StakingEventType::Bonded);

        // Locked 500 * UNITS as bonded on contracts's account
        assert_eq!(contract.frozen_balance(), 500 * ONE_TOKEN);
        // assert_eq!(contract_frozen_balance(&contract), 500 * ONE_TOKEN);

        reset_system_events();

        // Sending `unbond` message

        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Unbond")
            .add_arg(200 * ONE_TOKEN)
            .send_and_run_one_block();

        assert!(result.is_ok());

        assert_staking_events(&contract, 200 * ONE_TOKEN, StakingEventType::Unbonded);

        // The funds are still locked
        assert_eq!(
            contract.frozen_balance(),
            500 * ONE_TOKEN
        );

        // Pretend we have run the chain for at least the `unbonding period` number of eras

        move_n_bonding_durations(1);

        // Sending `withdraw_unbonded` message

        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("WithdrawUnbonded")
            .send_and_run_one_block();

        assert!(result.is_ok());

        // 200 * UNITS have been released, 300 * UNITS remain locked
        assert_eq!(
            contract.frozen_balance(),
            300 * ONE_TOKEN
        );

        assert_staking_events(&contract, 200 * ONE_TOKEN, StakingEventType::Withdrawn);

        let contract_ledger = contract.stash_ledger(); 
        
        // Check the bounded contract tokens
        assert_eq!(contract_ledger.active, 300 * ONE_TOKEN);
    });
}

#[test]
fn set_payee_works() {
    init_logger();

    new_test_ext(vec![SIGNER]).execute_with(|| {
        let contract = Contract::upload_sails_contract::<()>()
            .signer(SIGNER)
            .salt("contract")
            .app_constructor_name("New")
            .wasm(WASM_BINARY)
            .upload();

        // Bond funds with the `payee`` set to contract's stash (default)
        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Bond")
            .add_arg(100 * ONE_TOKEN)
            .add_arg(RewardAccount::Program)
            .with_value(100 * ONE_TOKEN)
            .send_and_run_one_block();

        assert!(result.is_ok());

        assert_staking_events(&contract, 100 * ONE_TOKEN, StakingEventType::Bonded);

        // Assert the `payee` is set to contract's stash

        let contract_payee = contract.payee_ledger();

        assert_eq!(contract_payee, Some(pallet_staking::RewardDestination::Stash));

        // Set the `payee` to SIGNER
        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("SetPayee")
            .add_arg(RewardAccount::Custom(ActorId32::from(REWARD_PAYEE)))
            .send_and_run_one_block();

        // Assert the `payee` is now set to SIGNER
        let contract_payee = contract.payee_ledger();

        assert_eq!(
            contract_payee,
            Some(pallet_staking::RewardDestination::Account(u64::from_origin(REWARD_PAYEE.into_origin())))
        );
    });
}

#[test]
fn rebond_works() {
    init_logger();

    new_test_ext(vec![SIGNER]).execute_with(|| {
        let contract = Contract::upload_sails_contract::<()>()
            .signer(SIGNER)
            .salt("contract")
            .app_constructor_name("New")
            .wasm(WASM_BINARY)
            .upload();

        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Bond")
            .add_arg(500 * ONE_TOKEN)
            .add_arg(RewardAccount::Program)
            .with_value(500 * ONE_TOKEN)
            .send_and_run_one_block();

        assert!(result.is_ok());

        assert_staking_events(&contract, 500 * ONE_TOKEN, StakingEventType::Bonded);

        // Locked 500 * UNITS as bonded on contracts's account
        assert_eq!(contract.frozen_balance(), 500 * ONE_TOKEN);

        reset_system_events();

        // Sending `unbond` message
        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Unbond")
            .add_arg(400 * ONE_TOKEN)
            .send_and_run_one_block();

        assert!(result.is_ok());

        assert_staking_events(&contract, 400 * ONE_TOKEN, StakingEventType::Unbonded);

        // All the bonded funds are still locked
        assert_eq!(
            contract.frozen_balance(),
            500 * ONE_TOKEN
        );

        // However, the ledger has been updated
        let contract_stash = contract.stash_ledger();

        // total stake active amount from contract 
        assert_eq!(contract_stash.active, 100 * ONE_TOKEN);
        assert_eq!(contract_stash.unlocking.len(), 1);

        // Sending `rebond` message
        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Rebond")
            .add_arg(200 * ONE_TOKEN)
            .send_and_run_one_block();

        assert!(result.is_ok());

        // All the bonded funds are still locked
        assert_eq!(
            contract.frozen_balance(),
            500 * ONE_TOKEN
        );

        // However, the ledger has been updated again
        let contract_stash = contract.stash_ledger();

        assert_eq!(contract_stash.active, 300 * ONE_TOKEN);
        assert_eq!(contract_stash.unlocking.len(), 1);

        // Sending another `rebond` message, with `value` exceeding the unlocking amount
        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Rebond")
            .add_arg(300 * ONE_TOKEN)
            .send_and_run_one_block();

        assert!(result.is_ok());

        // All the bonded funds are still locked
        assert_eq!(
            contract.frozen_balance(),
            500 * ONE_TOKEN
        );

        // The ledger has been updated again, however, the rebonded amount was limited
        // by the actual unlocking amount - not the `value` sent in the message.
        let contract_stash = contract.stash_ledger();

        assert_eq!(contract_stash.active, 500 * ONE_TOKEN);
        assert_eq!(contract_stash.unlocking.len(), 0);
    });
}

#[test]
fn payout_stakers_works() { // rewards
    init_logger();

    let authorities = vec![
        (VAL_1_STASH, VAL_1_STASH_AUTH_ID),
        (VAL_2_STASH, VAL_2_STASH_AUTH_ID),
        (VAL_3_STASH, VAL_3_STASH_AUTH_ID)
    ];

    new_test_ext_with_authorities_and_sessions(
        vec![SIGNER, REWARD_PAYEE],
        authorities
    ).execute_with(|| {
        let contract = Contract::upload_sails_contract::<()>()
            .signer(SIGNER)
            .salt("contract")
            .app_constructor_name("New")
            .wasm(WASM_BINARY)
            .upload();

        // Only nominating one target
        // let targets: Vec<ActorId> = vec![VAL_1_STASH.cast()];
        let targets: Vec<ActorId32> = vec![ActorId32::from(VAL_1_STASH)];

        // Bonding a quarter of validator's stake for easier calculations
        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Bond")
            .add_arg((
                250 * ONE_TOKEN,
                // RewardAccount::Custom(REWARD_PAYEE.into_origin().into()) 
                RewardAccount::Custom(ActorId32::from(REWARD_PAYEE)) 
            ))
            .with_value(250 * ONE_TOKEN)
            .send_and_run_one_block();

        assert!(result.is_ok());

        assert_staking_events(&contract, 250 * ONE_TOKEN, StakingEventType::Bonded);

        // Nomintate the validator
        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Nominate")
            .add_arg(targets.clone())
            .send_and_run_one_block();

        assert!(result.is_ok());

        let contract_n = contract.nominators();

        assert_eq!(contract_n, vec![VAL_1_STASH]);

        let rewards_payee_initial_balance = balance_from_user(REWARD_PAYEE);
        // Initial fund of tokens (ENDOWMENT) is 1_000_000_000_000_000
        assert_eq!(rewards_payee_initial_balance, ENDOWMENT);

        // Actually run the chain for a few eras (5) to accumulate some rewards
        run_for_n_blocks(
            5 * SESSION_DURATION_IN_BLOCKS * sessions_per_era(),
            None,
        );

        reset_system_events();

        // Send `payout_stakers` message for an era for which the rewards should have been earned
        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("PayoutStakers")
            .add_arg((
                // ActorId::from_origin(VAL_1_STASH.into_origin()),
                ActorId32::from(VAL_1_STASH),
                1
            ))
            .gas_limit(300_000_000_000)
            .send_and_run_one_block();

        assert!(result.is_ok());

        // Expecting the nominator to have received 1/5 of the rewards
        assert_eq!(
            balance_from_user(REWARD_PAYEE),
            rewards_payee_initial_balance + (100 * ONE_TOKEN) / 5
        );
    });
}
