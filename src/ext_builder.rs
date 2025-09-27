
use frame_system::{self as system};
use sp_runtime::{
    Perbill,
    BuildStorage,
    testing::UintAuthorityId
};
use crate::types::{
    AccountId,
    Balance
};
use crate::mock::{
    Test,
    System
};

use crate::runtime;

#[derive(Default)]
pub struct ExtBuilder {
    // Si no usas sesión, puedes pasar None en las llaves.
    pub initial_authorities: Vec<(AccountId, Option<UintAuthorityId>)>,
    pub endowed_accounts: Vec<AccountId>,
    pub endowment: Balance,
    pub enable_sessions: bool,  
}

impl ExtBuilder {
    pub fn endowment(mut self, e: Balance) -> Self {
        self.endowment = e; 
        self
    }

    pub fn with_endowed_accounts(mut self, accs: Vec<AccountId>) -> Self {
        self.endowed_accounts = accs; 
        self
    }

    pub fn with_initial_authorities(
        mut self,
        auths: Vec<(AccountId, Option<UintAuthorityId>)>,
    ) -> Self {
        self.initial_authorities = auths; 
        self
    }

    pub fn with_sessions(mut self) -> Self {
        self.enable_sessions = true;
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let mut storage = system::GenesisConfig::<Test>::default()
            .build_storage()
            .expect("system genesis builds");

        // --- Balances ---

        let endowed_accounts_iter = self
            .endowed_accounts
            .iter()
            .map(|k| (*k, self.endowment));

        let endowed_initial_authorities = self
            .initial_authorities
            .iter()
            .map(|(initial_authority_address, _)| (*initial_authority_address, self.endowment));

        let balances = endowed_accounts_iter
            .chain(endowed_initial_authorities)
            .collect();

        pallet_balances::GenesisConfig::<Test> { balances }
            .assimilate_storage(&mut storage)
            .expect("balances genesis");


        // --- Session (optional) ---

        if self.enable_sessions {
            let keys = self.initial_authorities
                .iter()
                .map(|(authority_addr, session_key)| (
                    *authority_addr,     // stash
                    *authority_addr,     // controller
                    session_key
                        .clone()
                        .expect("Authorization without session key") // key
                ))
                .collect();

            pallet_session::GenesisConfig::<Test> {
                keys,
                ..Default::default()
            }
            .assimilate_storage(&mut storage)
            .expect("session genesis");
        }

        // --- Staking ---

        let stakers = self.initial_authorities
            .iter()
            .map(|(authority_addr, _)| (
                    *authority_addr, // stash
                    *authority_addr, // controller
                    self.endowment,
                    pallet_staking::StakerStatus::<AccountId>::Validator,
            ))
            .collect();

        let invulnerables = self.initial_authorities
            .iter()
            .map(|(authority_addr, _)| *authority_addr)
            .collect();

        pallet_staking::GenesisConfig::<Test> {
            validator_count: self.initial_authorities.len() as u32,
            minimum_validator_count: self.initial_authorities.len() as u32,
            stakers,
            invulnerables,
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        }
        .assimilate_storage(&mut storage)
        .expect("staking genesis");

        let mut ext: sp_io::TestExternalities = storage.into();

        ext.execute_with(|| {
            let new_blk = 1;
            System::set_block_number(new_blk);
            runtime::on_initialize(new_blk);
        });
        
        ext
    }
}
