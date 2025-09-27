extern crate alloc;

use alloc::vec::Vec;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use super::actorid32::ActorId32;

/// Type that should be used to create a message to the staking built-in actor.
///
/// A `partial` mirror of the staking pallet interface. Not all extrinsics
/// are supported, more can be added as needed for real-world use cases.
// #[derive(Debug, Clone, Eq, PartialEq, Encode, Decode, TypeInfo)]

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode, TypeInfo)]
pub enum Request {
    /// Bond up to the `value` from the sender to self as the controller.
    #[codec(index = 0)]
    Bond { value: u128, payee: RewardAccount },

    /// Add up to the `value` to the sender's bonded amount.
    #[codec(index = 1)]
    BondExtra { value: u128 },

    /// Unbond up to the `value` to allow withdrawal after undonding period.
    #[codec(index = 2)]
    Unbond { value: u128 },

    /// Withdraw unbonded chunks for which undonding period has elapsed.
    #[codec(index = 3)]
    WithdrawUnbonded { num_slashing_spans: u32 },

    /// Add sender as a nominator of `targets` or update the existing targets.
    #[codec(index = 4)]
    Nominate { targets: Vec<ActorId32> },

    /// Declare intention to `temporarily` stop nominating while still having funds bonded.
    #[codec(index = 5)]
    Chill,

    /// Request stakers payout for the given era.
    #[codec(index = 6)]
    PayoutStakers { validator_stash: ActorId32, era: u32 },

    /// Rebond a portion of the sender's stash scheduled to be unlocked.
    #[codec(index = 7)]
    Rebond { value: u128 },

    /// Set the reward destination.
    #[codec(index = 8)]
    SetPayee { payee: RewardAccount },
}

/// An account where the rewards should accumulate on.
///
/// A "mirror" of the staking pallet's `RewardDestination` enum.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Encode, Decode, TypeInfo)]
pub enum RewardAccount {
    /// Pay rewards to the sender's account and increase the amount at stake.
    Staked,
    /// Pay rewards to the sender's account (usually, the one derived from `program_id`)
    /// without increasing the amount at stake.
    Program,
    /// Pay rewards to a custom account.
    Custom(ActorId32),
    /// Opt for not receiving any rewards at all.
    None,
}
