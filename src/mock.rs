use frame_support_test::TestRandomness;
use sp_runtime::{
    Perbill, 
    Permill, 
    traits::{
        BlakeTwo256, 
        IdentityLookup
    },
};
use frame_support::{
    PalletId, construct_runtime,
    parameter_types,
    traits::{
        ConstU32,
        ConstU64,
        ConstBool,
        FindAuthor,
    }
};
use frame_election_provider_support::{
    onchain,
    bounds::{ElectionBounds, ElectionBoundsBuilder}
};
use pallet_session::historical::{ self as pallet_session_historical };
use sp_core::{H256, crypto::key_types};
use sp_runtime::{
    KeyTypeId, 
    traits::OpaqueKeys,
    testing::UintAuthorityId
};
use sp_staking::SessionIndex;
use frame_system::{self as system};


use pallet_gear_builtin::{
    proxy,
    bls12_381,
    staking,
    ActorWithId
};

use crate::staking_helper::{
    FixedEraPayout,
    OnChainSeqPhragmen
};

use crate::types::{
    AccountId,
    BlockNumber,
    Balance,
    Block,
    BLOCK_AUTHOR,
    EXISTENTIAL_DEPOSIT,
    UNITS,
    SESSION_DURATION_IN_BLOCKS
};

// Configure a mock runtime to test the pallet.
construct_runtime!(
    pub enum Test
    {
        System: system,
        Balances: pallet_balances,
        Authorship: pallet_authorship,
        Timestamp: pallet_timestamp,
        Session: pallet_session, // staking
        Historical: pallet_session_historical, //staking
        Staking: pallet_staking,
        Proxy: pallet_proxy,
        GearProgram: pallet_gear_program,
        GearMessenger: pallet_gear_messenger,
        GearScheduler: pallet_gear_scheduler,
        GearBank: pallet_gear_bank,
        Gear: pallet_gear,
        GearGas: pallet_gear_gas,
        GearBuiltin: pallet_gear_builtin,
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
    pub ElectionBoundsOnChain: ElectionBounds = ElectionBoundsBuilder::default().build();
}

parameter_types! {
    pub const MinimumPeriod: u64 = 1_500;
    pub const BlockGasLimit: u64 = 100_000_000_000_000; // 100_000_000_000_000 of gas limit
    pub const OutgoingLimit: u32 = 1024;
    pub const OutgoingBytesLimit: u32 = 64 * 1024 * 1024;
    pub ReserveThreshold: BlockNumber = 1;
    pub GearSchedule: pallet_gear::Schedule<Test> = <pallet_gear::Schedule<Test>>::default();
    pub RentFreePeriod: BlockNumber = 12_000;
    pub RentCostPerBlock: Balance = 11;
    pub ResumeMinimalPeriod: BlockNumber = 100;
    pub ResumeSessionDuration: BlockNumber = 1_000;
    pub const PerformanceMultiplier: u32 = 100;
    pub const BankPalletId: PalletId = PalletId(*b"py/gbank");
    pub const GasMultiplier: common::GasMultiplier<Balance, u64> = common::GasMultiplier::ValuePerGas(100);
}

common::impl_pallet_system!(Test);
common::impl_pallet_balances!(Test);
common::impl_pallet_authorship!(Test, EventHandler = Staking);
common::impl_pallet_timestamp!(Test);
common::impl_pallet_staking!(
    Test,
    EraPayout = FixedEraPayout::<{ 100 * UNITS }>,
    NextNewSession = Session,
    ElectionProvider = onchain::OnChainExecution<OnChainSeqPhragmen>,
    GenesisElectionProvider = onchain::OnChainExecution<OnChainSeqPhragmen>,
);


pub struct TestSessionHandler;
impl pallet_session::SessionHandler<AccountId> for TestSessionHandler {
    const KEY_TYPE_IDS: &'static [KeyTypeId] = &[key_types::DUMMY];

    fn on_new_session<Ks: OpaqueKeys>(
        _changed: bool,
        _validators: &[(AccountId, Ks)],
        _queued_validators: &[(AccountId, Ks)],
    ) {
    }

    fn on_disabled(_validator_index: u32) {}

    fn on_genesis_session<Ks: OpaqueKeys>(_validators: &[(AccountId, Ks)]) {}
}

parameter_types! {
    pub const Period: u64 = SESSION_DURATION_IN_BLOCKS;
    pub const Offset: u64 = SESSION_DURATION_IN_BLOCKS + 1;
}


impl pallet_session::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = AccountId;
    type ValidatorIdOf = pallet_staking::StashOf<Self>;
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type SessionManager = pallet_session_historical::NoteHistoricalRoot<Self, Staking>;
    type SessionHandler = TestSessionHandler;
    type Keys = UintAuthorityId;
    type WeightInfo = ();
}

impl pallet_session_historical::Config for Test {
    type FullIdentification = pallet_staking::Exposure<AccountId, u128>;
    type FullIdentificationOf = pallet_staking::ExposureOf<Test>;
}

pallet_gear_bank::impl_config!(Test);
pallet_gear_gas::impl_config!(Test);
pallet_gear_scheduler::impl_config!(Test);
pallet_gear_program::impl_config!(Test);
pallet_gear_messenger::impl_config!(Test, CurrentBlockNumber = Gear);
pallet_gear::impl_config!(
    Test,
    Schedule = GearSchedule,
    BuiltinDispatcherFactory = GearBuiltin,
);


impl pallet_gear_builtin::Config for Test {
    type RuntimeCall = RuntimeCall;
    type Builtins = (
        ActorWithId<1, bls12_381::Actor<Self>>,
        ActorWithId<2, staking::Actor<Self>>,
        ActorWithId<4, proxy::Actor<Self>>,

    );
    type BlockLimiter = GearGas;
    type WeightInfo = ();
}
