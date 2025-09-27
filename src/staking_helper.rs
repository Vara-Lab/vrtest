use frame_election_provider_support::{
    onchain,
    SequentialPhragmen,
    bounds::{
        ElectionBounds,
        ElectionBoundsBuilder
    }
};
use frame_support::parameter_types;
use sp_runtime::{
    Perbill,
    traits::ConstU32,
};
use sp_staking::SessionIndex;
use crate::mock::{
    Test,
    Staking
};
use crate::types::{
    AccountId,
    Balance,
    EXISTENTIAL_DEPOSIT
};


parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
    pub ElectionBoundsOnChain: ElectionBounds = ElectionBoundsBuilder::default().build();
}

// Fixed payout for each era
pub struct FixedEraPayout<const PAYOUT: u128>;
impl<const PAYOUT: u128> pallet_staking::EraPayout<u128> for FixedEraPayout<PAYOUT> {
    fn era_payout(
        _total_staked: u128,
        _total_issuance: u128,
        _era_duration_millis: u64,
    ) -> (u128, u128) {
        (PAYOUT, 0)
    }
}

pub struct OnChainSeqPhragmen;

impl onchain::Config for OnChainSeqPhragmen {
    type System = Test;
    type Solver = SequentialPhragmen<AccountId, Perbill>;
    type DataProvider = Staking;
    type WeightInfo = ();
    type MaxWinners = ConstU32<100>;
    type Bounds = ElectionBoundsOnChain;
}