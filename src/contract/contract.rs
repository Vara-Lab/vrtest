use pallet_staking::{RewardDestination, StakingLedger};
use gprimitives::ActorId;
use pallet_balances::AccountData;
use sp_staking::StakingAccount;
use parity_scale_codec::Encode;
use crate::mock::{
    System, 
    Test
};
use super::builders::{
    CommandCall,
    // QueryCall,
    CalculateGasCall,
    UploadSailsWasm,
    UploadWasm,
    UploadCode,
    CreateContract
};

pub struct Contract {
    /// ## Contract address
    pub(crate) address: ActorId,
    /// ## System account
    /// Is the account associted with the account (tokens, deposits, staking, etc.)
    pub(crate) account: u64
}

impl Contract {
    pub fn new(address: ActorId, account: u64) -> Self {
        Self {
            address,
            account
        }
    }

    pub fn address(&self) -> ActorId {
        self.address
    }

    pub fn create_contract<R: Encode>() -> CreateContract<R> {
        CreateContract::new()
    }

    pub fn upload_code() -> UploadCode {
        UploadCode::default()
    }

    pub fn upload_contract() -> UploadWasm {
        UploadWasm::default()
    }

    pub fn upload_sails_contract<R: Encode>() -> UploadSailsWasm<R> {
        UploadSailsWasm::new()
    }

    pub fn new_command(&self) -> CommandCall {
        CommandCall::new(self.address.clone())
    }

    // pub fn new_query<R: Encode>(&self) -> QueryCall<R> {
    //     QueryCall::new(self.address.clone())
    // }

    pub fn new_calculate_gas(&self) -> CalculateGasCall {
        CalculateGasCall::new(self.address.clone())
    }

    /// ## Contract's stash ledger
    /// Get the ledger associated with the contract stash account. You can check contract data like the amount
    /// of tokens staked by the contract, etc.
    pub fn stash_ledger(&self) -> StakingLedger<Test> {
        let ledger = pallet_staking::Pallet::<Test>::ledger(
            StakingAccount::Stash(self.account)
        ).unwrap();

        ledger
    }

    /// ## Contract's payee ledger
    /// Get the ledger associated with the contract payee account.
    pub fn payee_ledger(&self) -> Option<RewardDestination<u64>> {
        let payee = pallet_staking::Pallet::<Test>::payee(
            StakingAccount::Stash(self.account)
        );

        payee
    }


    /// ## Return contract nominations
    pub fn nominators(&self) -> Vec<u64> {
        let targets_before = pallet_staking::Nominators::<Test>::get(self.account)
                .map_or_else(Vec::new, |x| x.targets.into_inner());

        targets_before
    }

    /// ## Contract free balance
    /// Returns the free tokens of the smart contract. If staked, the value will be: contract_balance - staked_value
    pub fn free_balance(&self) -> u128 {
        let account_data = System::account(self.account);
        account_data.data.free
    }

    /// ## Contract frozen balance
    /// Frozen balance that is locked or staked
    pub fn frozen_balance(&self) -> u128 {
        let account_data = System::account(self.account);
        account_data.data.frozen
    }

    /// ## Contract account data
    /// Returns more balance data from a contract
    pub fn get_account_data(&self) -> AccountData<u128> {
        let account_data: AccountData<u128> = System::account(self.account).data;

        account_data
    }
    
}