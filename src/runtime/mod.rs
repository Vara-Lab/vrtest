use frame_system::{
    pallet_prelude::BlockNumberFor,
    limits::BlockWeights,
};
use gear_core_errors::{ErrorReplyReason, ReplyCode, SimpleExecutionError};
use sp_runtime::testing::UintAuthorityId;
use frame_support::{
    pallet_prelude::{
        DispatchClass,
        Weight
    },
    traits::{
        OnFinalize, 
        OnInitialize,
        EstimateNextSessionRotation, 
        Get
    },
    assert_ok
};
use common::{storage::Limiter, GasTree, MessageId, Origin};
use gear_core::ids::{prelude::{ActorIdExt, CodeIdExt}, CodeId};
use gprimitives::ActorId;
use pallet_gear::Event as GearEvent;
use pallet_gear_builtin::GasAllowanceOf;

use crate::mock::{
    Period,
    Offset,
    Authorship, 
    Balances, 
    Gear, 
    GearBank, 
    GearBuiltin, 
    GearGas, 
    GearMessenger, 
    GearProgram, 
    GearScheduler, 
    Historical, 
    Proxy, 
    RuntimeEvent, 
    RuntimeOrigin, 
    Session, 
    Staking, 
    System, 
    Test, 
    Timestamp
};
use crate::types::{
    BlockWeightsOf,
    QueueOf,
    GasTreeOf,
    GasHandlerOf,
    AccountId,
    Balance,
    StakingEventType,
    DEFAULT_GAS_LIMIT,
    ENDOWMENT,
    MILLISECS_PER_BLOCK,
};
use crate::runtime_types::*;
use crate::ext_builder::ExtBuilder;

use crate::contract::Contract;

use parity_scale_codec::Encode;

/// Account (u64) to actorId
pub fn u64_to_actorid(account: u64) -> ActorId {
    ActorId::from(account)
}

/// Account (u64) to account id (into_origin)
pub fn u64_to_origin_u64(account: u64) -> u64 {
    u64::from_origin(account.into_origin())
}

/// Bloque duration in ms 
pub fn block_in_ms() -> u64 {
    <Test as pallet_timestamp::Config>::MinimumPeriod::get().saturating_mul(2)
}

/// Session duration in blocks
pub fn session_duration_in_blocks() -> u64 {
    Period::get()
}
/// Initial offset in blocks
pub fn session_offset_in_blocks() -> u64 {
    Offset::get()
}

/// Sessions per era
pub fn sessions_per_era() -> u64 {
    <Test as pallet_staking::Config>::SessionsPerEra::get() as u64
}

/// Era duration in blocks - 1_250
pub fn era_duration_in_blocks() -> u64 {
    session_duration_in_blocks().saturating_mul(sessions_per_era())
}

/// Era duration in milliseconds - 1_500 * 1000
pub fn era_duration_ms() -> u128 {
    (era_duration_in_blocks() as u128).saturating_mul(block_in_ms() as u128)
}

/// ## Get the current block number
pub fn current_block() -> u64 {
    System::block_number()
}

/// Current index
pub fn current_session_index() -> u32 {
    pallet_session::Pallet::<Test>::current_index()
}

/// current era
pub fn current_era() -> u32 {
    Staking::current_era().unwrap()
}

/// Index of active era
pub fn current_era_index() -> u32 {
    pallet_staking::Pallet::<Test>::active_era().unwrap().index
}

/// Estimated block to the next sesion rotation 
pub fn next_session_rotation_block() -> Option<u64> {
    let now = current_block();
    let (block, __) = <Test as pallet_session::Config>::NextSessionRotation::estimate_next_session_rotation(now);

    block
}

/// ## Function to go to the next finished bonding duration
/// Use this function if you want to rapidly pass the bonding duration, if you use it, features like rewards
/// will not work correctly or with expected results
pub fn move_n_bonding_durations(bonding_durations: u32) {
    pallet_staking::CurrentEra::<Test>::put(
        <Test as pallet_staking::Config>::BondingDuration::get() + bonding_durations,
    );
}

/// ## Return contract nominations
pub fn contract_nominators(contract: &Contract) -> Vec<u64> {
    let targets_before = pallet_staking::Nominators::<Test>::get(contract.account)
            .map_or_else(Vec::new, |x| x.targets.into_inner());

    targets_before
}

/// ## Run to next block
pub fn run_to_next_block() {
    run_for_n_blocks(1, None)
}

/// ## Run to next n blocks
pub fn run_for_n_blocks(n: u64, remaining_weight: Option<u64>) {
    let now = System::block_number();
    let until = now + n;
    for current_blk in now..until {
        if let Some(remaining_weight) = remaining_weight {
            GasAllowanceOf::<Test>::put(remaining_weight);
            let max_block_weight = <BlockWeightsOf<Test> as Get<BlockWeights>>::get().max_block;
            System::register_extra_weight_unchecked(
                max_block_weight.saturating_sub(Weight::from_parts(remaining_weight, 0)),
                DispatchClass::Normal,
            );
        }

        let max_block_weight = <BlockWeightsOf<Test> as Get<BlockWeights>>::get().max_block;
        System::register_extra_weight_unchecked(max_block_weight, DispatchClass::Mandatory);
        Gear::run(frame_support::dispatch::RawOrigin::None.into(), None).unwrap();

        on_finalize(current_blk);

        let new_block_number = current_blk + 1;
        System::set_block_number(new_block_number);
        on_initialize(new_block_number);
    }
}

/// ## User balance
/// Returns the user balance.
/// 
/// To know the balance of a contract use: contract_free_balance
pub fn balance_from_user(address: u64) -> u128 {
    Balances::free_balance(address)
}

/// ## Upload the sails contract wasm
/// This function will upload the given wasm (&[u8]) with a initial payload and will return a Contract 
/// struct that can be used to check contrect data, send message to it, etc.
/// 
/// And, at the end, it will run to the next block
pub fn upload_sails_wasm<T: Encode>(
    signer: AccountId,
    wasm: &[u8], 
    init_payload: T, 
    salt: &[u8], 
    gas_limit: Option<u64>
) -> Contract {
    let contract_id = ActorId::generate_from_user(CodeId::generate(wasm), b"contract");
    let contract_account_id = u64::from_origin(contract_id.into_origin());
    let gas_limit = gas_limit.unwrap_or(DEFAULT_GAS_LIMIT);

    assert_ok!(
        Gear::upload_program(
            RuntimeOrigin::signed(signer), 
            wasm.to_vec(), 
            salt.to_vec(), 
            init_payload.encode(), 
            gas_limit, 
            0, 
            false
        )
    );

    run_to_next_block();

    // [TODO]:

    Contract::new(contract_id, contract_account_id)
}

/// ## Upload the contract wasm
/// This function will upload the given wasm (&[u8]) and will return a Contract struct that can be used to 
/// check contrect data, send message to it, etc.
/// 
/// And, at the end, it will run to the next block
pub fn upload_wasm(
    wasm: &[u8], 
    signer: AccountId, 
    salt: &[u8], 
    gas_limit: Option<u64>
) -> Contract {
    let contract_id = ActorId::generate_from_user(CodeId::generate(wasm), b"contract");
    let contract_account_id = u64::from_origin(contract_id.into_origin());
    let gas_limit = gas_limit.unwrap_or(DEFAULT_GAS_LIMIT);

    assert_ok!(
        Gear::upload_program(
            RuntimeOrigin::signed(signer), 
            wasm.to_vec(), 
            salt.to_vec(), 
            Default::default(), 
            gas_limit, 
            0, 
            false
        )
    );

    run_to_next_block();

    // [TODO]:

    Contract::new(contract_id, contract_account_id)
}

// Run on_initialize hooks in order as they appear in AllPalletsWithSystem.
pub(crate) fn on_initialize(new_block_number: BlockNumberFor<Test>) {
    // 
    System::on_initialize(new_block_number);

    // Timestamp::set_timestamp(new_block_number.saturating_mul(MILLISECS_PER_BLOCK));
    Timestamp::set_timestamp(new_block_number.saturating_mul(block_in_ms()));

    // Authorship::on_initialize(new_block_number);
    // GearGas::on_initialize(new_block_number);
    // GearMessenger::on_initialize(new_block_number);
    // Gear::on_initialize(new_block_number);
    // GearBank::on_initialize(new_block_number);


    Authorship::on_initialize(new_block_number);
    Session::on_initialize(new_block_number);
    Historical::on_initialize(new_block_number);
    Staking::on_initialize(new_block_number);
    Proxy::on_initialize(new_block_number);
    GearProgram::on_initialize(new_block_number);
    GearMessenger::on_initialize(new_block_number);
    GearScheduler::on_initialize(new_block_number);
    GearBank::on_initialize(new_block_number);
    Gear::on_initialize(new_block_number);
    GearGas::on_initialize(new_block_number);
    GearBuiltin::on_initialize(new_block_number);
}

// Run on_finalize hooks (in pallets reverse order, as they appear in AllPalletsWithSystem)
pub(crate) fn on_finalize(current_blk: BlockNumberFor<Test>) {
    // Authorship::on_finalize(current_blk);
    // Gear::on_finalize(current_blk);
    // GearBank::on_finalize(current_blk);

    GearBuiltin::on_finalize(current_blk);
    GearGas::on_finalize(current_blk);
    Gear::on_finalize(current_blk);
    GearBank::on_finalize(current_blk);
    GearScheduler::on_finalize(current_blk);
    GearMessenger::on_finalize(current_blk);
    GearProgram::on_finalize(current_blk);
    Proxy::on_finalize(current_blk);
    Staking::on_finalize(current_blk);
    Historical::on_finalize(current_blk);
    Session::on_finalize(current_blk);
    Authorship::on_finalize(current_blk);

    assert!(!System::events().iter().any(|e| {
        matches!(
            e.event,
            RuntimeEvent::Gear(pallet_gear::Event::QueueNotProcessed)
        )
    }))
}

/// ## Init logger
pub fn init_logger() {
    let _ = tracing_subscriber::fmt::try_init();
}

/// ## Reset system events
pub fn reset_system_events() {
    System::reset_events();
}

pub fn current_stack() -> Vec<ExecutionTraceFrame> {
    DEBUG_EXECUTION_TRACE.with(|stack| stack.borrow().clone())
}

pub fn in_transaction() -> bool {
    IN_TRANSACTION.with(|value| *value.borrow())
}

pub fn set_transaction_flag(new_val: bool) {
    IN_TRANSACTION.with(|value| *value.borrow_mut() = new_val)
}

pub fn message_queue_empty() -> bool {
    QueueOf::<Test>::iter_keys().next().is_none()
}

pub fn gas_tree_empty() -> bool {
    GasTreeOf::<Test>::iter_keys().next().is_none()
        && <GasHandlerOf<Test> as GasTree>::total_supply() == 0
}

pub fn message_id_fom_message_sent(
    signer: u64,
    contract_id: ActorId
) -> Option<MessageId> {
    let mut msg_id = None;

    for ev in System::events() {
        if let RuntimeEvent::Gear(GearEvent::MessageQueued { id, source, destination, .. }) = &ev.event { // [TODO]: check for entry field update
            if *source == signer && *destination == contract_id {
                msg_id = Some(*id);
            }
        }
    }

    msg_id
}

#[track_caller]
pub fn assert_staking_events(contract: &Contract, balance: Balance, t: StakingEventType) {
    assert!(System::events().into_iter().any(|e| {
        match e.event {
            RuntimeEvent::Staking(pallet_staking::Event::<Test>::Bonded { stash, amount }) => {
                t == StakingEventType::Bonded && stash == contract.account && balance == amount
            }
            RuntimeEvent::Staking(pallet_staking::Event::<Test>::Unbonded {
                stash,
                amount,
            }) => t == StakingEventType::Unbonded && stash == contract.account && balance == amount,
            RuntimeEvent::Staking(pallet_staking::Event::<Test>::Withdrawn {
                stash,
                amount,
            }) => t == StakingEventType::Withdrawn && stash == contract.account && balance == amount,
            _ => false,
        }
    }))
}

#[track_caller]
pub fn assert_no_staking_events() {
    assert!(
        System::events()
            .into_iter()
            .all(|e| { !matches!(e.event, RuntimeEvent::Staking(_)) })
    )
}

#[track_caller]
pub fn assert_error_message_sent() {
    assert!(System::events().into_iter().any(|e| {
        match e.event {
            RuntimeEvent::Gear(pallet_gear::Event::UserMessageSent { message, .. }) => {
                match message.details() {
                    Some(details) => {
                        details.to_reply_code()
                            == ReplyCode::Error(ErrorReplyReason::Execution(
                                SimpleExecutionError::UserspacePanic,
                            ))
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }))
}

#[track_caller]
pub fn assert_payload_contains(s: &'static str) {
    assert!(System::events().into_iter().any(|e| {
        match e.event {
            RuntimeEvent::Gear(pallet_gear::Event::UserMessageSent { message, .. }) => {
                let s_bytes = s.as_bytes();
                message
                    .payload_bytes()
                    .windows(s_bytes.len())
                    .any(|window| window == s_bytes)
            }
            _ => false,
        }
    }))
}

pub fn start_transaction() {
    sp_externalities::with_externalities(|ext| ext.storage_start_transaction())
        .expect("externalities should exists");

    set_transaction_flag(true);
}

pub fn rollback_transaction() {
    sp_externalities::with_externalities(|ext| {
        ext.storage_rollback_transaction()
            .expect("ongoing transaction must be there");
    })
    .expect("externalities should be set");

    set_transaction_flag(false);
}


/// ## Create a new runtime test
/// This function will init the tests, you need to pass the address that will receive tokens (1000 tokens)
pub fn new_test_ext(addresses_to_fund_tokens: Vec<u64>) -> sp_io::TestExternalities {
    let bank_address = GearBank::bank_address();

    let mut endowed_accounts = vec![bank_address];

    endowed_accounts.extend(addresses_to_fund_tokens.iter());
    endowed_accounts.extend(GearBuiltin::list_builtins());

    ExtBuilder::default()
        .endowment(ENDOWMENT)
        .with_endowed_accounts(endowed_accounts)
        .build()
}

/// ## Create a new runtime test
/// This function will init the tests, you need to pass the address that will receive tokens (1000 tokens)
/// and initial authorities
pub fn new_test_ext_with_authorities(
    addresses_to_fund_tokens: Vec<u64>,
    initial_authorities: Vec<u64>
) -> sp_io::TestExternalities {
    let bank_address = GearBank::bank_address();

    let mut endowed_accounts = vec![bank_address];

    endowed_accounts.extend(addresses_to_fund_tokens.iter());
    endowed_accounts.extend(GearBuiltin::list_builtins());

    let initial_authorities = initial_authorities
        .into_iter()
        .map(|authority_address| (authority_address, None))
        .collect();

    ExtBuilder::default()
        .endowment(ENDOWMENT)
        .with_endowed_accounts(endowed_accounts)
        .with_initial_authorities(initial_authorities)
        .build()
}

// /// ## Create a new runtime test 
// /// This function will init the tests, you need to pass the address that will receive tokens (1000 tokens),
// /// this function will enable session to manage rewards from staking, etc.
// pub fn new_test_ext_with_sessions(
//     addresses_to_fund_tokens: Vec<u64>,
// ) -> sp_io::TestExternalities {
//     let bank_address = GearBank::bank_address();

//     let mut endowed_accounts = vec![bank_address];

//     endowed_accounts.extend(addresses_to_fund_tokens.iter());
//     endowed_accounts.extend(GearBuiltin::list_builtins());

//     ExtBuilder::default()
//         .endowment(ENDOWMENT)
//         .with_endowed_accounts(endowed_accounts)
//         .with_sessions()
//         .build()
// }

/// ## Create a new runtime test 
/// This function will init the tests, you need to pass the address that will receive tokens (1000 tokens),
/// and initial authorities, this function will enable session to manage rewards from staking, etc.
pub fn new_test_ext_with_authorities_and_sessions(
    addresses_to_fund_tokens: Vec<u64>,
    initial_authorities: Vec<(u64, u64)>
) -> sp_io::TestExternalities {
    let bank_address = GearBank::bank_address();

    let mut endowed_accounts = vec![bank_address];

    endowed_accounts.extend(addresses_to_fund_tokens.iter());
    endowed_accounts.extend(GearBuiltin::list_builtins());

    let initial_authorities = initial_authorities
        .into_iter()
        .map(|(authority_address, authority_auth_id)| (authority_address, Some(UintAuthorityId(authority_auth_id))))
        .collect();

    ExtBuilder::default()
        .endowment(ENDOWMENT)
        .with_endowed_accounts(endowed_accounts)
        .with_initial_authorities(initial_authorities)
        .with_sessions()
        .build()
}









