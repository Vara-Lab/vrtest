use parity_scale_codec::{
    Encode, 
    Decode
};
use common::Origin;
use gprimitives::ActorId;
use sp_core::H256;
use crate::{
    mock::Gear,
    types::ContractQueryError,
    runtime
};

// /// ## Read state 
// /// Read state from a query method from a contract, yo need to provide the type to decode the result
pub struct QueryCall<T: Encode> {
    contract_address: ActorId,
    service_name: Option<String>,
    method_name: Option<String>,
    gas_allowance: Option<u64>,
    no_sails_query: bool,
    payload: Option<T>
}

impl<T: Encode> QueryCall<T> {
    pub fn new(contract_address: ActorId) -> Self {
        QueryCall {
            contract_address,
            service_name: None,
            method_name: None,
            gas_allowance: None,
            no_sails_query: false,
            payload: None,
        }
    }

    pub fn no_sails_query(mut self) -> Self {
        self.no_sails_query = true;

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

    pub fn payload(mut self, payload: T) -> Self {
        self.payload = Some(payload);

        self
    }

    pub fn gas_allowance(mut self, gas_allowance: u64) -> Self {
        self.gas_allowance = Some(gas_allowance);

        self
    }

    pub fn send<R: Decode>(self) -> Result<R, ContractQueryError> {
        if self.service_name.is_none() && !self.no_sails_query {
            panic!("Service name is not set!");
        }

        if self.method_name.is_none() && !self.no_sails_query{
            panic!("Service method name is not set!");
        }

        let user_payload = if self.payload.is_some() {
            self.payload.unwrap().encode()
        } else {
            ().encode()
        };

        let payload = if !self.no_sails_query {
            [
                self.service_name.unwrap().encode(),
                self.method_name.unwrap().encode(),
                user_payload
            ]
            .concat()
        } else {
            user_payload
        };

        let result = Gear::read_state(
            self.contract_address.into(), 
            payload, 
            self.gas_allowance
        );

        if let Err(error) = result {
            let msg_decode = String::from_utf8(error);

            match msg_decode {
                Ok(message) => return Err(ContractQueryError::ReadStateError(message)),
                Err(e) => panic!("Cant decode query error mesage: {:?}", e)
            };
        }

        let decode_result = <(String, String, R)>::decode(&mut &result.unwrap()[..]);

        if let Err(error) = decode_result {
            return Err(ContractQueryError::ResultDecodeError(format!("{:?}", error)));
        }

        Ok(decode_result.unwrap().2)
    }
}