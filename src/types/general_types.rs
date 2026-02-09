use crate::mock::Test;
use common::{event::DispatchStatus, GasProvider};
use sp_runtime::DispatchError;

pub type AccountId = u64;
pub type BlockNumber = u64;
pub type Balance = u128;
pub type Block = frame_system::mocking::MockBlock<Test>;
pub type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
pub type BlockWeightsOf<T> = <T as frame_system::Config>::BlockWeights;

pub(crate) type QueueOf<T> = pallet_gear_messenger::Dispatches<T>;
pub(crate) type GasHandlerOf<T> = <<T as pallet_gear::Config>::GasProvider as GasProvider>::GasTree;
pub(crate) type GasTreeOf<T> = pallet_gear_gas::GasNodes<T>;

pub(crate) const BLOCK_AUTHOR: AccountId = 10; // [TODO]: given by val_1_stash

pub(crate) const EXISTENTIAL_DEPOSIT: u128 = 1 * UNITS;
pub const ENDOWMENT: u128 = 1_000 * UNITS;

pub(crate) const UNITS: u128 = 1_000_000_000_000; // 10^(-12) precision
#[allow(unused)]
pub(crate) const MILLISECS_PER_BLOCK: u64 = 3_000;

pub(crate) const DEFAULT_GAS_LIMIT: u64 = 20_000_000_000;

// Staking consts

pub const SESSION_DURATION_IN_BLOCKS: u64 = 2_400; // 250;

#[derive(PartialEq)]
pub enum StakingEventType {
    Bonded,
    Unbonded,
    Withdrawn,
}

// public consts

pub const ONE_TOKEN: u128 = UNITS;
pub const CONTRACT_EXISTENCIAL_DEPOSIT: u128 = EXISTENTIAL_DEPOSIT;

// Contract util data

/// ## Error in query calls to contracts
#[derive(Debug)]
pub enum ContractQueryError {
    QueryError(DispatchError),
    ReadStateError(String),
    ResultDecodeError(String),
}

/// ## Error in command calls to contracts
#[derive(Debug)]
pub enum ContractCommandError {
    CommandError(DispatchError),
    ResultDecodeError(String),
    Failed(DispatchStatus),
    Error(String),
    TimeOut,
}

/// ## Command response
pub enum ContractResponse<R> {
    Response(R),
    OkNoReply,
    Waited,
}
