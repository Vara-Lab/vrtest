use gear_core::ids::{prelude::{ActorIdExt}, CodeId};
use parity_scale_codec::Encode;
use frame_support::assert_ok;
use gprimitives::ActorId;
use common::Origin;
use crate::{
    mock::{
        Gear,
        RuntimeOrigin
    },
    types::DEFAULT_GAS_LIMIT,
    contract::Contract,
    runtime
};

pub struct CreateContract<T: Encode> {
    signer: Option<u64>,
    code_id: Option<CodeId>,
    constructor_name: Option<String>,
    init_payload: Option<T>,
    salt: Option<Vec<u8>>,
    keep_alive: bool,
    gas_limit: Option<u64>
}

impl<T: Encode> CreateContract<T> {
    pub fn new() -> Self {
        Self {
            signer: None,
            code_id: None,
            constructor_name: None,
            init_payload: None,
            salt: None,
            keep_alive: false,
            gas_limit: None
        }
    }

    pub fn app_constructor_name(mut self, constructor_name: &'static str) -> Self {
        self.constructor_name = Some(constructor_name.to_string());

        self
    }

    pub fn init_payload(mut self, payload: T) -> Self {
        self.init_payload = Some(payload);

        self
    } 

    pub fn signer(mut self, signer: u64) -> Self {
        self.signer = Some(signer);

        self
    }

    pub fn code_id(mut self, code_id: CodeId) -> Self {
        self.code_id = Some(code_id);

        self
    }

    pub fn salt(mut self, salt: &'static str) -> Self {
        self.salt = Some(salt.as_bytes().to_vec());

        self
    }

    pub fn gas_limit(mut self, gas_limit: u64) -> Self {
        self.gas_limit = Some(gas_limit);

        self
    }

    pub fn keep_alive(mut self) -> Self {
        self.keep_alive = true;
        
        self
    }

    pub fn create(self) -> Contract {
        if self.signer.is_none() {
            panic!("Signer is not set!");
        }

        if self.code_id.is_none() {
            panic!("Wasm not set!!");
        }

        if self.salt.is_none() {
            panic!("Salt not set!");
        }

        if self.constructor_name.is_none() {
            panic!("Constructor name not set!");
        }

        let signer = self.signer.unwrap();
        let code_id = self.code_id.unwrap();
        let gas_limit = self.gas_limit.unwrap_or(DEFAULT_GAS_LIMIT);
        let payload = if self.init_payload.is_some() {
            (
                self.constructor_name.unwrap(),
                self.init_payload.unwrap()
            ).encode()
        } else {
            (
                self.constructor_name.unwrap(),
                ()
            ).encode()
        };

        let (contract_id, contract_account) = gen_contract_ids(
            code_id.clone(), 
            &self.salt.as_ref().unwrap()[..]
        );

        assert_ok!(
            Gear::create_program(
                RuntimeOrigin::signed(signer), 
                code_id, 
                self.salt.unwrap(), 
                payload, 
                gas_limit, 
                0, 
                self.keep_alive
            )
        );

        runtime::run_to_next_block();

        Contract::new(contract_id, contract_account)
    }
}

fn gen_contract_ids(code_id: CodeId, salt: &[u8]) -> (ActorId, u64) {
    let contract_id = ActorId::generate_from_user(code_id, salt);
    let contract_account_id = u64::from_origin(contract_id.into_origin());

    (contract_id, contract_account_id)
}