use parity_scale_codec::Encode;
use frame_support::assert_ok;
use common::Origin;
use gprimitives::ActorId;
use gear_core::ids::{prelude::{ActorIdExt, CodeIdExt}, CodeId};
use crate::{
    mock::{
        Gear,
        RuntimeOrigin
    },
    types::DEFAULT_GAS_LIMIT,
    contract::Contract,
    runtime
};

pub trait UploadWasmT: Sized {
    fn signer(self, signer: u64) -> Self;
    fn wasm(self, wasm: &[u8]) -> Self;
    fn salt(self, salt: &'static str) -> Self;
    fn gas_limit(self, gas_limit: u64) -> Self;
    fn keep_alive(self) -> Self;
    fn upload(self) -> Contract;
}

pub struct UploadSailsWasm<T: Encode> {
    signer: Option<u64>,
    wasm: Option<Vec<u8>>,
    constructor_name: Option<String>,
    init_payload: Option<T>,
    salt: Option<Vec<u8>>,
    keep_alive: bool,
    gas_limit: Option<u64>
}

impl<T: Encode> UploadSailsWasm<T> {
    pub fn new() -> Self {
        Self {
            signer: None,
            wasm: None,
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
}

impl<T: Encode> UploadWasmT for UploadSailsWasm<T> {
    fn signer(mut self, signer: u64) -> Self {
        self.signer = Some(signer);

        self
    }

    fn wasm(mut self, wasm: &[u8]) -> Self {
        self.wasm = Some(wasm.to_vec());

        self
    }

    fn salt(mut self, salt: &'static str) -> Self {
        self.salt = Some(salt.as_bytes().to_vec());

        self
    }

    fn gas_limit(mut self, gas_limit: u64) -> Self {
        self.gas_limit = Some(gas_limit);

        self
    }

    fn keep_alive(mut self) -> Self {
        self.keep_alive = true;
        
        self
    }

    fn upload(self) -> Contract {
        if self.signer.is_none() {
            panic!("Signer is not set!");
        }

        if self.wasm.is_none() {
            panic!("Wasm not set!!");
        }

        if self.salt.is_none() {
            panic!("Salt not set!");
        }

        if self.constructor_name.is_none() {
            panic!("Constructor name not set!");
        }

        let (contract_id, contract_account) = gen_contract_ids(
            &self.wasm.as_ref().unwrap()[..], 
            &self.salt.as_ref().unwrap()[..]
        );

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

        assert_ok!(
            Gear::upload_program(
                RuntimeOrigin::signed(self.signer.unwrap()), 
                self.wasm.unwrap(),//wasm.to_vec(), 
                self.salt.unwrap(), // salt.to_vec(), 
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


#[derive(Default)]
pub struct UploadWasm {
    signer: Option<u64>,
    wasm: Option<Vec<u8>>,
    salt: Option<Vec<u8>>,
    keep_alive: bool,
    gas_limit: Option<u64>
}

impl UploadWasmT for UploadWasm {
    fn signer(mut self, signer: u64) -> Self {
        self.signer = Some(signer);

        self
    }

    fn wasm(mut self, wasm: &[u8]) -> Self {
        self.wasm = Some(wasm.to_vec());

        self
    }

    fn salt(mut self, salt: &'static str) -> Self {
        self.salt = Some(salt.as_bytes().to_vec());

        self
    }

    fn gas_limit(mut self, gas_limit: u64) -> Self {
        self.gas_limit = Some(gas_limit);

        self
    }

    fn keep_alive(mut self) -> Self {
        self.keep_alive = true;

        self
    }

    fn upload(self) -> Contract {
        if self.signer.is_none() {
            panic!("Signer is not set!");
        }

        if self.wasm.is_none() {
            panic!("Wasm not set!!");
        }

        if self.salt.is_none() {
            panic!("Salt not set!");
        }

        let (contract_id, contract_account) = gen_contract_ids(
            &self.wasm.as_ref().unwrap()[..], 
            &self.salt.as_ref().unwrap()[..]
        );

        let gas_limit = self.gas_limit.unwrap_or(DEFAULT_GAS_LIMIT);

        assert_ok!(
            Gear::upload_program(
                RuntimeOrigin::signed(self.signer.unwrap()), 
                self.wasm.unwrap(),
                self.salt.unwrap(), 
                Default::default(), 
                gas_limit, 
                0, 
                self.keep_alive
            )
        );

        runtime::run_to_next_block();

        Contract::new(contract_id, contract_account)
    }
}

fn gen_contract_ids(wasm: &[u8], salt: &[u8]) -> (ActorId, u64) {
    let contract_id = ActorId::generate_from_user(CodeId::generate(wasm), salt);
    let contract_account_id = u64::from_origin(contract_id.into_origin());

    (contract_id, contract_account_id)
}