use sp_runtime::{
    RuntimeDebug,
    traits::BlakeTwo256
};
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use gbuiltin_proxy::ProxyType as BuiltinProxyType;
use frame_support::{
    traits::InstanceFilter,
    parameter_types
};
use crate::types::Balance;
use crate::mock::{
    Test,
    Balances,
    RuntimeCall,
    RuntimeEvent
};

#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Encode,
    Decode,
    RuntimeDebug,
    MaxEncodedLen,
    scale_info::TypeInfo,
)]
pub enum ProxyType {
    Any,
    NonTransfer,
    Governance,
    Staking,
    IdentityJudgement,
    CancelProxy,
}

impl Default for ProxyType {
    fn default() -> Self {
        Self::Any
    }
}

impl From<BuiltinProxyType> for ProxyType {
    fn from(proxy_type: BuiltinProxyType) -> Self {
        match proxy_type {
            BuiltinProxyType::Any => ProxyType::Any,
            BuiltinProxyType::NonTransfer => ProxyType::NonTransfer,
            BuiltinProxyType::Governance => ProxyType::Governance,
            BuiltinProxyType::Staking => ProxyType::Staking,
            BuiltinProxyType::IdentityJudgement => ProxyType::IdentityJudgement,
            BuiltinProxyType::CancelProxy => ProxyType::CancelProxy,
        }
    }
}

impl InstanceFilter<RuntimeCall> for ProxyType {
    fn filter(&self, c: &RuntimeCall) -> bool {
        match self {
            ProxyType::Any => true,
            ProxyType::NonTransfer => !matches!(c, RuntimeCall::Balances(..)),
            ProxyType::CancelProxy => {
                matches!(
                    c,
                    RuntimeCall::Proxy(pallet_proxy::Call::reject_announcement { .. })
                )
            }
            ProxyType::Staking => matches!(c, RuntimeCall::Staking(..)),
            ProxyType::Governance | ProxyType::IdentityJudgement => {
                unimplemented!("No pallets defined in test runtime")
            }
        }
    }
    fn is_superset(&self, o: &Self) -> bool {
        match (self, o) {
            (x, y) if x == y => true,
            (ProxyType::Any, _) => true,
            (_, ProxyType::Any) => false,
            (ProxyType::NonTransfer, _) => true,
            _ => false,
        }
    }
}

parameter_types! {
    pub const ProxyDepositBase: Balance = 1;
    pub const ProxyDepositFactor: Balance = 1;
    pub const MaxProxies: u32 = 100;
    pub const MaxPending: u32 = 100;
    pub const AnnouncementDepositBase: Balance = 1;
    pub const AnnouncementDepositFactor: Balance = 1;
}

impl pallet_proxy::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type Currency = Balances;
    type ProxyType = ProxyType;
    type ProxyDepositBase = ProxyDepositBase;
    type ProxyDepositFactor = ProxyDepositFactor;
    type MaxProxies = MaxProxies;
    type WeightInfo = ();
    type MaxPending = MaxPending;
    type CallHasher = BlakeTwo256;
    type AnnouncementDepositBase = AnnouncementDepositBase;
    type AnnouncementDepositFactor = AnnouncementDepositBase;
}