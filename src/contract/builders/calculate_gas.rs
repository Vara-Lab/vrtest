use parity_scale_codec::Encode;
use common::Origin;
use gprimitives::ActorId;
use crate::{
    mock::{
        Test,
        Gear,
    },
    runtime
};

#[derive(Debug, Clone)]
pub struct GasEstimation {
    pub min_limit: u128,
    pub burned: u128,
    pub reserved: u128,
    pub may_be_returned: u64,
    pub waited: bool,
}

pub struct CalculateGasCall {
    contract_address: ActorId,
    signer: Option<u64>, 
    service_name: Option<String>, 
    method_name: Option<String>, 
    value: u128, 
    allow_other_panics: bool, 
    initial_gas: Option<u64>,
    gas_allowance: Option<u64>,
    no_sails_command: bool,
    payload: Vec<u8>
}

impl CalculateGasCall {
    pub fn new(contract_address: ActorId) -> Self {
        Self {
            contract_address,
            signer: None,
            service_name: None,
            method_name: None,
            value: 0,
            allow_other_panics: false,
            initial_gas: None,
            gas_allowance: None,
            no_sails_command: false,
            payload: vec![]
        }
    }

    pub fn no_sails_command(mut self) -> Self {
        self.no_sails_command = true;

        self
    }

    pub fn signer(mut self, signer: u64) -> Self {
        self.signer = Some(signer);

        self
    }

    pub fn service_name(mut self, service_name: &'static str) -> Self {
        self.service_name = Some(service_name.to_string());

        self
    }

    pub fn method_name(mut self, method_name: &'static str) -> Self {
        self.method_name = Some(method_name.to_string());

        self
    }

    pub fn with_value(mut self, value: u128) -> Self {
        self.value = value;

        self
    }

    pub fn allow_other_panics(mut self) -> Self {
        self.allow_other_panics = true;

        self
    }

    pub fn initial_gas(mut self, initial_gas: u64) -> Self {
        self.initial_gas = Some(initial_gas);

        self
    }

    pub fn gas_allowance(mut self, gas_allowance: u64) -> Self {
        self.gas_allowance = Some(gas_allowance);

        self
    }

    /// ## Add an argument to the payload
    /// This method will add arguments into the payload (args that you can find in your .idl)
    pub fn add_arg(mut self, arg: impl Encode) -> Self {
        arg.encode_to(&mut self.payload);

        self
    }

    pub fn check_data(&self) {
        if self.service_name.is_none() && !self.no_sails_command {
            panic!("Service name is not set!");
        }

        if self.method_name.is_none() && !self.no_sails_command {
            panic!("Service method name is not set!");
        }

        if self.signer.is_none() {
            panic!("Signer cant be empty!");
        }
    }

    /// ## Calculate gas
    /// This functions will calculate the gas fees to send a message to the given contract
    pub fn calculate_gas(self) -> GasEstimation { // u128
        self.check_data();

        runtime::start_transaction();

        let payload = if !self.no_sails_command {
            [
                self.service_name.unwrap().encode(),
                self.method_name.unwrap().encode(),
                self.payload
            ]
            .concat()
        } else {
            self.payload
        };

        // let user_payload = if self.payload.is_some() {
        //     self.payload.unwrap().encode()
        // } else {
        //     ().encode()
        // };

        // let payload = if !self.no_sails_command {
        //     [
        //         self.service_name.unwrap().encode(),
        //         self.method_name.unwrap().encode(),
        //         user_payload
        //     ]
        //     .concat()
        // } else {
        //     user_payload
        // };

        let res = Gear::calculate_gas_info(
            self.signer.unwrap().into_origin(), 
            pallet_gear::manager::HandleKind::Handle(self.contract_address), 
            payload, 
            self.value, 
            self.allow_other_panics, 
            self.initial_gas, 
            self.gas_allowance
        );


        let info = res.map_err(|e| String::from_utf8(e).unwrap_or_else(|_| "calculate_gas_info failed".into()));

        if let Err(error) = info {
            panic!("{}", error);
            // let tmp = String::from_utf8(error);
            // let message_error = if let Ok(message) = tmp {
            //     message
            // } else {
            //     "calculate_gas_info failed!".to_string()
            // };

            // panic!("{message_error}");
        }

        runtime::rollback_transaction();

        let info = info.unwrap();

        GasEstimation {
            min_limit: Self::gas_price(info.min_limit),
            burned: Self::gas_price(info.burned),
            reserved: Self::gas_price(info.reserved),
            may_be_returned: info.may_be_returned,
            waited: info.waited,
        }

        // Self::gas_price(res.unwrap().burned)
    }

    pub fn gas_price(gas: u64) -> u128 {
        <Test as pallet_gear_bank::Config>::GasMultiplier::get().gas_to_value(gas)
    }
}
