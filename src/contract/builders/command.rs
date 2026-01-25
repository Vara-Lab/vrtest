use parity_scale_codec::{
    Encode, 
    Decode
};
use common::{Origin, event::DispatchStatus};
use gprimitives::ActorId;
use pallet_gear::Event as GearEvent;
use crate::{
    mock::{
        Gear,
        RuntimeOrigin,
        RuntimeEvent,
        System,
    },
    types::{
        ContractCommandError,
        ContractResponse,
        DEFAULT_GAS_LIMIT
    },
    runtime
};


// /// ## Read state 
// /// Read state from a query method from a contract, yo need to provide the type to decode the result

pub struct CommandCall {
    contract_address: ActorId,
    signer: Option<u64>,
    service_name: Option<String>,
    method_name: Option<String>,
    gas_limit: Option<u64>,
    keep_alive: bool,
    value: u128,
    max_blocks_to_wait: u64,
    no_sails_command: bool,
    is_query: bool,
    get_waited: bool,
    payload: Vec<u8>
}

impl CommandCall {
    pub fn new(contract_address: ActorId) -> Self {
        Self {
            contract_address,
            signer: None,
            service_name: None,
            method_name: None,
            gas_limit: None,
            keep_alive: false,
            no_sails_command: false,
            value: 0,
            max_blocks_to_wait: 5,
            is_query: false,
            get_waited: true,
            payload: vec![],
        }
    }

    pub fn skip_waited(mut self) -> Self {
        self.get_waited = false;

        self
    }
 
    /// ## Transform te command to a query
    /// This attribute is importante because to get the contract state with a query, you need to
    /// send a message to the contract, so, calling this method will roll back any change, and the
    /// signer balances will be the same
    pub fn transform_to_query(mut self) -> Self {
        self.is_query = true;

        self
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

    pub fn gas_limit(mut self, gas_limit: u64) -> Self {
        self.gas_limit = Some(gas_limit);
        
        self
    }

    pub fn keep_alive(mut self) -> Self {
        self.keep_alive = true;

        self
    }

    pub fn with_value(mut self, value: u128) -> Self {
        self.value = value;

        self
    }

    pub fn max_blocks_to_wait(mut self, max_blocks_to_wait: u64) -> Self {
        self.max_blocks_to_wait = max_blocks_to_wait;

        self
    }

    fn check_data(&self) {
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

    /// ## Add an argument to the payload
    /// This method will add arguments into the payload (args that you can find in your .idl)
    pub fn add_arg(mut self, arg: impl Encode) -> Self {
        arg.encode_to(&mut self.payload);

        self
    }

    /// ## Send message to a contract and run one block
    /// Send a message to the given contract, if gas_limit not provided, it will use the Default value: 20_000_000_000
    /// 
    /// This function will increment the current block by one block.
    pub fn send_and_run_one_block(self) -> Result<(), ContractCommandError> {
        self.send()?;

        runtime::run_to_next_block();

        Ok(())
    }

    /// ## Send message to a contract
    /// Send a message to the given contract, if gas_limit not provided, it will use the Default value: 20_000_000_000
    pub fn send(self) -> Result<(), ContractCommandError> {
        self.check_data();

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

        let result = Gear::send_message(
            RuntimeOrigin::signed(self.signer.unwrap()), 
            self.contract_address, 
            payload, 
            self.gas_limit.unwrap_or(DEFAULT_GAS_LIMIT), 
            self.value, 
            self.keep_alive
        );

        if let Err(error) = result {
            return Err(ContractCommandError::CommandError(error.error));
        }

        Ok(())
    }

    /// ## Send a message and wait for contract reply
    /// Send a message to the given contract, you need to provide the return type to be able to decode the result.
    /// 
    /// If gas limit not provided, it will use the Default value: 20_000_000_000
    /// 
    /// This function will increment the current block until it find the response from the contract 
    /// 
    /// > IMPORTANT: This function will reset system events in order to find the contract response
    pub fn send_recv<R: Decode>(
        self
    ) -> Result<ContractResponse<R>, ContractCommandError> {
        self.check_data();

        let is_query = self.is_query;

        if is_query { runtime::start_transaction(); }

        runtime::reset_system_events();

        let signer = self.signer
            .clone()
            .unwrap();
        let signer_origin = ActorId::from_origin(signer.into_origin());
        let contract_address = self.contract_address
            .clone();
        let mut max_blocks_to_wait = self.max_blocks_to_wait;
        let get_waited = self.get_waited;

        self.send_and_run_one_block()?;

        let msg_id = runtime::message_id_fom_message_sent(
            signer, 
            contract_address
        ).ok_or(ContractCommandError::TimeOut)?;

        while max_blocks_to_wait > 0 {
            for e in System::events() {
                if let RuntimeEvent::Gear(gear_event) = &e.event {
                    match gear_event {
                        GearEvent::UserMessageSent { message, .. } => {
                            if message.destination() == signer_origin && message.source() == contract_address {
                                let response = <(String, String, R)>::decode(
                                    &mut &message.payload_bytes()[..]
                                ).map_err(|e| ContractCommandError::ResultDecodeError(e.to_string()))?;

                                if is_query { runtime::rollback_transaction(); }
                                
                                runtime::run_to_next_block();

                                return Ok(ContractResponse::Response(response.2));
                            }
                        },
                        GearEvent::MessagesDispatched { statuses, .. } => {
                            if let Some(status) = statuses.get(&msg_id) {
                                match status {
                                    DispatchStatus::Success => {
                                        return Ok(ContractResponse::OkNoReply);
                                    }
                                    other => return Err(
                                        ContractCommandError::Failed(other.clone())
                                    ),
                                }
                            }
                        },
                        GearEvent::MessageWaited { id, .. } if *id == msg_id => {
                            if get_waited {
                                return Ok(ContractResponse::Waited);
                            }
                        },
                        _ => {}
                    }
                }
            }

            runtime::run_to_next_block();
            max_blocks_to_wait -= 1;
        }

        if is_query { runtime::rollback_transaction(); }

        Ok(ContractResponse::OkNoReply)
    }

    /// ## Send a message to a contract and check for errors
    /// Send a message to the given contract, if gas limit not provided, it will use the Default value: 20_000_000_000
    /// 
    /// This function will increment the current block until it find the result of the transaction, if reaches the 
    /// max block to wait, it will return an error.
    pub fn send_check_result(
        self, 
        mut max_blocks_to_wait: u64
    ) -> Result<(), ContractCommandError>{
        self.check_data();

        runtime::reset_system_events();

        let signer = self.signer
            .clone()
            .unwrap();
        
        let contract_address = self.contract_address
            .clone();

        self.send_and_run_one_block()?;

        let msg_id = runtime::message_id_fom_message_sent(
            signer, 
            contract_address
        ).ok_or(ContractCommandError::TimeOut)?;

        while max_blocks_to_wait > 0 {
            for e in System::events() {
                if let RuntimeEvent::Gear(gear_event) = &e.event {
                    match gear_event {
                        GearEvent::MessagesDispatched { statuses, .. } => {
                            if let Some(status) = statuses.get(&msg_id) {
                                match status {
                                    DispatchStatus::Success => {
                                        return Ok(());
                                    }
                                    other => return Err(
                                        ContractCommandError::Failed(other.clone())
                                    ),
                                }
                            }
                        }
                        // si cayó a waitlist, avanza más bloques hasta woken + dispatched
                        GearEvent::MessageWaited { id, .. } if *id == msg_id => {
                            return Ok(());
                        },
                        _ => {}
                    }
                }
            }

            runtime::run_to_next_block();
            max_blocks_to_wait -= 1;
        }

        Err(ContractCommandError::TimeOut)
    }

}