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
pub fn upload_create_contract_with_code_id() {
    init_logger();

    new_test_ext(vec![SIGNER]).execute_with(|| {
        let contract_code_id = Contract::upload_code()
            .signer(SIGNER)
            .wasm(WASM_BINARY)
            .upload();

        let contract = Contract::create_contract()
            .signer(SIGNER)
            .salt("contract-hello")
            .app_constructor_name("New")
            .code_id(contract_code_id)
            .init_payload(())
            .create();

        let result = contract.new_command()
            .signer(SIGNER)
            .service_name("ContractService")
            .method_name("Hello")
            .add_arg(())
            .send_recv::<String>();

        if let Err(error) = result {
            println!("{:?}", error);
            panic!("Command error");
        }

        let ContractResponse::Response(response) = result.unwrap() else {
            panic!("Incorrect response");
        };

        assert_eq!(response, format!("Hello {}", ActorId::from(SIGNER.into_origin())));
    });
}