use common::{ActorId, Origin};
use vrtest::{
    contract::{Contract, builders::UploadWasmT},
    runtime::*,
    types::{
        ContractResponse,
        ONE_TOKEN, 
    }
};

use contract::WASM_BINARY;
const SIGNER: u64 = 1;

#[test]
pub fn hello_send_message() {
    init_logger();

    new_test_ext(vec![SIGNER]).execute_with(|| {
        let contract = Contract::upload_sails_contract()
            .signer(SIGNER)
            .salt("contract")
            .app_constructor_name("New")
            .init_payload(())
            .wasm(WASM_BINARY)
            .upload();

        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Hello")
            .payload(())
            .send();

        assert!(result.is_ok());
    });
}

#[test]
pub fn hello_send_message_get_reply() {
    init_logger();

    new_test_ext(vec![SIGNER]).execute_with(|| {{
        // With turbofish because it does not includes payload
        let contract = Contract::upload_sails_contract::<()>()
            .signer(SIGNER)
            .salt("contract")
            .app_constructor_name("New")
            .wasm(WASM_BINARY)
            .upload();

        // With turbofish to avoid .payload(()) call because the payload function set the type 
        // of the payload to send to the contract
        let x = contract.new_command::<()>()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Hello")
            // set the max blocks to wait to the contract, if not set, it will wait 5 blocks
            .max_blocks_to_wait(10)
            // Set the return type from the contract
            .send_recv::<String>();

        if let Err(error) = x {
            println!("{:?}", error);
            panic!("Command error");
        }

        let ContractResponse::Response(response) = x.unwrap() else {
            panic!("Incorrect response");
        };

        assert_eq!(response, format!("Hello {}", ActorId::from(SIGNER.into_origin())));
    }});
}

#[test]
pub fn send_value() {
    init_logger();

    new_test_ext(vec![SIGNER]).execute_with(|| {{
        let contract = Contract::upload_sails_contract::<()>()
            .signer(SIGNER)
            .salt("contract")
            .app_constructor_name("New")
            .wasm(WASM_BINARY)
            .upload();

        let user_balance = balance_from_user(SIGNER);

        let _gas_fees = contract.new_calculate_gas::<()>()
            .signer(SIGNER)
            .with_value(ONE_TOKEN)
            .service_name("ContractService")
            .method_name("SendValue")
            .calculate_gas();

        assert_eq!(
            user_balance,
            balance_from_user(SIGNER)
        );

        let result = contract.new_command::<()>()
            .signer(SIGNER)
            .with_value(ONE_TOKEN)
            .service_name("ContractService")
            .method_name("SendValue")
            .send_recv::<String>();

        assert!(result.is_ok());

        let ContractResponse::Response(response) = result.unwrap() else {
            panic!("Incorrect response");
        };

        assert_eq!(response, format!("Value get: {}", ONE_TOKEN));

        assert_eq!(
            contract.free_balance(),
            ONE_TOKEN * 2
        );
    }});
}

#[test]
pub fn get_set_counter() {
    init_logger();

    new_test_ext(vec![SIGNER]).execute_with(|| {{
        let contract = Contract::upload_sails_contract::<()>()
            .signer(SIGNER)
            .salt("contract")
            .app_constructor_name("New")
            .wasm(WASM_BINARY)
            .upload();


        let _gas_fees = contract.new_calculate_gas::<()>()
            .signer(SIGNER)
            .with_value(ONE_TOKEN)
            .service_name("ContractService")
            .method_name("Increment")
            .calculate_gas();

        let result = contract.new_command::<()>()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Increment")
            .send_and_run_one_block();

        assert!(result.is_ok());

        let _gas_fees = contract.new_calculate_gas::<()>()
            .signer(SIGNER)
            .with_value(ONE_TOKEN)
            .service_name("ContractService")
            .method_name("CounterValue")
            .calculate_gas();

        let signer_balance = balance_from_user(SIGNER);

        assert_eq!(
            signer_balance,
            balance_from_user(SIGNER)
        );

        let result = contract.new_command::<()>()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("CounterValue")
            // Transform the commando into a query
            .transform_to_query()
            .send_recv::<u64>();

        if let Err(error) = result {
            println!("{error:?}");
            panic!("Panic reading state!");
        }

        let ContractResponse::Response(response) = result.unwrap() else {
            panic!("Incorrect response!");
        };

        assert_eq!(
            1,
            response
        );

        assert_eq!(
            signer_balance,
            balance_from_user(SIGNER)
        )
    }});
}


