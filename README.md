# vrtest

vrtest is a testing crate that will help you test your smart contracts, with the feature that a "node" is raised which, from mocks, you can access and modify aspects such as the current block, advance n blocks, send messages to built-in actors, etc. Thus, vrtest gives you a broader approach to how to test your smart contracts for [Vara Network](https://vara.network/) without the need to raise a node.

## 📋 Content

- [vrtests limitations](#vrtests_limitations_section)
- [Suggested contract to use](#suggested_contract_to_use_section)
- [Installation](#installation_section)
- [Runtime functions](#runtime_functions_section)
    - [new_test_ext](#new_test_ext_function)
    - [new_test_ext_with_authorities_and_sessions](#new_test_ext_with_authorities_and_sessions_function)
    - [init_logger](#init_logger_function)
    - [run_to_next_block](#run_to_next_block_function)
    - [run_for_n_blocks](#run_for_n_blocks_function)
    - [reset_system_events](#reset_system_events_function)
    - [block_in_ms](#block_in_ms_function)
    - [session_duration_in_blocks](#session_duration_in_blocks_function)
    - [sessions_per_era](#sessions_per_era_function)
    - [era_duration_in_blocks](#era_duration_in_blocks_function)
    - [era_duration_ms](#era_duration_ms_function)
    - [current_timestamp](#current_timestamp_function)
    - [current_block](#current_block_function)
    - [current_session_index](#current_session_index_function)
    - [current_era](#current_era_function)
    - [balance_from_user](#balance_from_user_function)
- [Runtime types](#runtime_types_section)
- [Contract Functions](#contract_functions_sections)
    - [Methods](#contract-methods)
    - [Builders](#builders)
    - [Upload a contract](#upload-a-contract)
    - [Calculate gas](#calculate-gas)
    - [Send a command](#send-a-command)

<a id="vrtests_limitations_section"></a>

## 🚧 vrtests considerations

vrtests has some considerations, such as the times handled within the runtime:

- The time of each block is *3000 ms* (same as `testnet` and `mainnet`).
- Each session lasts *2_400 blocks* (or 2 hours, check the [wiki](https://wiki.vara.network/docs/staking) for more details).
- Each era lasts 6 sessions, however, at the beginning (genesis) the first era will last 5 sessions, the following ones will last 6 sessions each era.
- `Block transition`: You need to handle by yourself the block transition in the runtime, some functions in vrtests 

<a id="suggested_contract_to_use_section"></a>

## 💡 Suggested contract to use

When creating your smart contract, it is recommended that you use the smart contract that is in the [Vara Lab repository](https://github.com/Vara-Lab/Base-Smart-Contract), this is made so that you can program and test your contract without problems with gclient, gtest and unit tests using Syscalls mocks, and at the same time, in test it gives you the wasm_binary of your contract which is used for testing with vrtest.

<a id="installation_section"></a>

## 📦 Installation

The purpose of this crate is to be used only for testing, so you must import the crate as follows in your Cargo.toml in the "dev-dependencies" section,
and set a `patch` to work with same dependencias as vrtest:

```toml
[dev-dependencies]
vrtest = { git = "https://github.com/Vara-Lab/vrtest.git" }
# more crates ...

[patch.crates-io] #patch
gsys = { git = "https://github.com/gear-tech/gear.git", tag = "v1.10.0" }
gstd = { git = "https://github.com/gear-tech/gear.git", tag = "v1.10.0" }
gear-core = { git = "https://github.com/gear-tech/gear.git", tag = "v1.10.0" }
gear-core-errors = { git = "https://github.com/gear-tech/gear.git", tag = "v1.10.0" }
gprimitives = { git = "https://github.com/gear-tech/gear.git", tag = "v1.10.0" }
gclient = { git = "https://github.com/gear-tech/gear.git", tag = "v1.10.0" }
gtest = { git = "https://github.com/gear-tech/gear.git", tag = "v1.10.0" }
gbuiltin-staking = { git = "https://github.com/gear-tech/gear.git", tag = "v1.10.0" }
gear-common = { git = "https://github.com/gear-tech/gear.git", tag = "v1.10.0" }
gear-wasm-builder = { git = "https://github.com/gear-tech/gear.git", tag = "v1.10.0" }
gcore = { git = "https://github.com/gear-tech/gear.git", tag = "v1.10.0" }

```

Finally, you have to work rust 1.91 because of some sails crates.

<a id="runtime_functions_section"></a>

## ⚙️ Runtime functions

vrtests has many functions that you can use to change the current block, find out what session you're in, what era you're in, a user's balance, etc. In the runtime module, you can find the following functions:

<a id="new_test_ext_function"></a>

- `new_test_ext`: This function is one of the most important, since it generates the "test runtime", to this as an argument you must pass the ids of the users who will sign (u64 data), each id that is passed will be funded with 1000 tokens, then you have to call the `execute_with` function which receives a closure where all the test code will go. Example:

    ```rust
    use vrtest::runtime::new_test_ext;

    const SIGNER: u64 = 1;

    #[test]
    pub fn init_runtime_test() {
        // Will fund 1000 Tokens to SIGNER
        new_test_ext(vec![SIGNER]).execute_with(|| {
            // test logic ...
        });
    }
    ```
    
<a id="new_test_ext_with_authorities_and_sessions_function"></a>

- `new_test_ext_with_authorities_and_sessions`: This function is the second most important, since like the previous function (new_test_ext), it also starts the runtime tests, and at the same time, sessions and validators will be established here, which will start the staking process as well as the process of counting sessions and eras, in order to test these characteristics and test the built-in staking actor. Example:

    ```rust
    use vrtest::runtime::new_test_ext;

    const SIGNER: u64 = 1;
    const VAL_1_STASH: u64 = 10;
    const VAL_1_STASH_AUTH_ID: u64 = 11;

    #[test]
    pub fn init_runtime_test() {
        let authorities = vec![
            (VAL_1_STASH, VAL_1_STASH_AUTH_ID),
        ];
        new_test_ext_with_authorities_and_sessions(
            vec![SIGNER], // Will fund 1000 Tokens to SIGNER
            authorities   // This will act as validators that you can nominate in your contracts
        ).execute_with(|| {
            // test logic ...
        });
    }
    ```

<a id="init_logger_function"></a>

- `init_logger`: This function will init the logfer for the runtime. Example:

    ```rust
    use vrtest::runtime::*;

    const SIGNER: u64 = 1;

    #[test]
    pub fn init_runtime_test() {
        init_logger(); // Init the logger

        // Will fund 1000 Tokens to SIGNER
        new_test_ext(vec![SIGNER]).execute_with(|| {
            // test logic ...
        });
    }
    ```

<a id="run_to_next_block_function"></a>

- `run_to_next_block`: This function will go the next block in the runtime. Example:

    ```rust
    use vrtest::runtime::*;

    const SIGNER: u64 = 1;

    #[test]
    pub fn init_runtime_test() {
        init_logger(); // Init the logger

        // Will fund 1000 Tokens to SIGNER
        new_test_ext(vec![SIGNER]).execute_with(|| { // or new_test_ext_with_authorities_and_sessions
            run_to_next_block();
            // Current block: 2
            // test logic ...
        });
    }
    ```

<a id="run_for_n_blocks_function"></a>

- `run_for_n_blocks`: This function will traverse the specified number of blocks (Traversing blocks affects time as well as staking times). Example: 

    ```rust
    use vrtest::runtime::*;

    const SIGNER: u64 = 1;

    #[test]
    pub fn init_runtime_test() {
        init_logger(); // Init the logger

        // Will fund 1000 Tokens to SIGNER
        new_test_ext(vec![SIGNER]).execute_with(|| { // or new_test_ext_with_authorities_and_sessions
            run_for_n_blocks(100, None);
            // Current block: 101
            // test logic ...
        });
    }
    ```

<a id="reset_system_events_function"></a>

- `reset_system_events`: When you send a message to a contract, or the contract make an action (send a message, stake some tokens, etc), the runtime test will store this events, so, this function will reset the events so that you can better manage the next events that happen. Example:

    ```rust
    use vrtest::runtime::*;

    const SIGNER: u64 = 1;

    #[test]
    pub fn init_runtime_test() {
        init_logger(); // Init the logger

        // Will fund 1000 Tokens to SIGNER
        new_test_ext(vec![SIGNER]).execute_with(|| { // or new_test_ext_with_authorities_and_sessions
            reset_system_events(); // Reset system events
            // test logic ...
        });
    }
    ```

<a id="block_in_ms_function"></a>

- `block_in_ms`: Block duration in ms. Example:

    ```rust
    use vrtest::runtime::*;

    const SIGNER: u64 = 1;

    #[test]
    pub fn init_runtime_test() {
        init_logger(); // Init the logger

        // Will fund 1000 Tokens to SIGNER
        new_test_ext(vec![SIGNER]).execute_with(|| { // or new_test_ext_with_authorities_and_sessions
            assert_eq!(
                block_in_ms(), // Block time in milliseconds
                3_000
            );

            // test logic ...
        });
    }
    ```

<a id="session_duration_in_blocks_function"></a>

- `session_duration_in_blocks`: session duration in blocks (2400 blocks). Example: 

    ```rust
    use vrtest::runtime::*;

    const SIGNER: u64 = 1;

    #[test]
    pub fn init_runtime_test() {
        init_logger(); // Init the logger

        // Will fund 1000 Tokens to SIGNER
        new_test_ext(vec![SIGNER]).execute_with(|| { // or new_test_ext_with_authorities_and_sessions
            assert_eq!(
                session_duration_in_blocks(),
                2_400
            );

            // test logic ...
        });
    }
    ```

<a id="sessions_per_era_function"></a>

- `sessions_per_era`: returns the number of sessions per era. Example:

    ```rust
    use vrtest::runtime::*;

    const SIGNER: u64 = 1;

    #[test]
    pub fn init_runtime_test() {
        init_logger(); // Init the logger

        // Will fund 1000 Tokens to SIGNER
        new_test_ext(vec![SIGNER]).execute_with(|| { // or new_test_ext_with_authorities_and_sessions
            assert_eq!(
                sessions_per_era(),
                6
            );

            // test logic ...
        });
    }
    ```

<a id="era_duration_in_blocks_function"></a>

- `era_duration_in_blocks`: Return the era duration in blocks (14_400). Example:

    ```rust
    use vrtest::runtime::*;

    const SIGNER: u64 = 1;

    #[test]
    pub fn init_runtime_test() {
        init_logger(); // Init the logger

        // Will fund 1000 Tokens to SIGNER
        new_test_ext(vec![SIGNER]).execute_with(|| { // or new_test_ext_with_authorities_and_sessions
            assert_eq!(
                era_duration_in_blocks(),
                14_400
            );

            // test logic ...
        });
    }
    ```

<a id="era_duration_ms_function"></a>

- `era_duration_ms`: Era duration in milliseconds (43_200_000). Example:

    ```rust
    use vrtest::runtime::*;

    const SIGNER: u64 = 1;

    #[test]
    pub fn init_runtime_test() {
        init_logger(); // Init the logger

        // Will fund 1000 Tokens to SIGNER
        new_test_ext(vec![SIGNER]).execute_with(|| { // or new_test_ext_with_authorities_and_sessions
            assert_eq!(
                era_duration_ms(),
                43_200_000
            );

            // test logic ...
        });
    }
    ```

<a id="current_timestamp_function"></a>

- `current_timestamp`: Returns the current timestamp. Example:

    ```rust
    use vrtest::runtime::*;

    const SIGNER: u64 = 1;

    #[test]
    pub fn init_runtime_test() {
        init_logger(); // Init the logger

        // Will fund 1000 Tokens to SIGNER
        new_test_ext(vec![SIGNER]).execute_with(|| { // or new_test_ext_with_authorities_and_sessions
            run_for_n_blocks(100, None);
            // Current block: 101

            assert_eq!(
                current_timestamp(), // Get the current block
                300_000
            );

            // test logic ...
        });
    }
    ```

<a id="current_block_function"></a>

- `current_block`: Returns the current block. Example:

    ```rust
    use vrtest::runtime::*;

    const SIGNER: u64 = 1;

    #[test]
    pub fn init_runtime_test() {
        init_logger(); // Init the logger

        // Will fund 1000 Tokens to SIGNER
        new_test_ext(vec![SIGNER]).execute_with(|| { // or new_test_ext_with_authorities_and_sessions
            run_for_n_blocks(100, None);
            // Current block: 101

            assert_eq!(
                current_block(), // Get the current block
                101
            );

            // test logic ...
        });
    }
    ```

<a id="current_session_index_function"></a>

- `current_session_index`: Get the current session index. Example:

    ```rust
    use vrtest::runtime::*;

    const SIGNER: u64 = 1;
    const VAL_1_STASH: u64 = 10;
    const VAL_1_STASH_AUTH_ID: u64 = 11;

    #[test]
    pub fn init_runtime_test() {
        init_logger(); // Init the logger

        let authorities = vec![
            (VAL_1_STASH, VAL_1_STASH_AUTH_ID),
        ];

        // Enable sessions with this function to be able to change the sessions and eras when 
        // you "run" n blocks
        new_test_ext_with_authorities_and_sessions(
            vec![SIGNER], // Will fund 1000 Tokens to SIGNER
            authorities   // This will act as validators that you can nominate in your contracts
        ).execute_with(|| {
            run_for_n_blocks(250, None);

            assert_eq!(
                current_block(),
                251
            );

            assert_eq!(
                current_session_index(), // Get the current session index
                1
            );

            // test logic ...
        });
    }
    ```

<a id="current_era_function"></a>

- `current_era` or `current_era_index`: This function returns the current era. Example:

    ```rust
    use vrtest::runtime::*;

    const SIGNER: u64 = 1;
    const VAL_1_STASH: u64 = 10;
    const VAL_1_STASH_AUTH_ID: u64 = 11;

    #[test]
    pub fn init_runtime_test() {
        init_logger(); // Init the logger

        let authorities = vec![
            (VAL_1_STASH, VAL_1_STASH_AUTH_ID),
        ];

        // Enable sessions with this function to be able to change the sessions and eras when 
        // you "run" n blocks
        new_test_ext_with_authorities_and_sessions(
            vec![SIGNER], // Will fund 1000 Tokens to SIGNER
            authorities   // This will act as validators that you can nominate in your contracts
        ).execute_with(|| {
            run_for_n_blocks(1_500, None);

            assert_eq!(
                current_block(),
                1_501
            );

            assert_eq!(
                current_era(), // Get the current session index
                1
            );

            // test logic ...
        });
    }
    ```

<a id="balance_from_user_function"></a>

- `balance_from_user`: Get the current user balance. Example:

    ```rust
    use vrtest::runtime::*;

    const SIGNER: u64 = 1;

    #[test]
    pub fn init_runtime_test() {
        init_logger(); // Init the logger

        // Will fund 1000 Tokens to SIGNER
        new_test_ext(vec![SIGNER]).execute_with(|| { // or new_test_ext_with_authorities_and_sessions
            assert_eq!(
                era_duration_in_blocks(),
                14_400
            );

            // test logic ...
        });
    }
    ```

<a id="runtime_types_section"></a>

## 📚 Runtime types

Vrtest has some types that can be useful with your tests, if you will tests your contract with vrtest, you have to use only the types provided by the crate in some cases:

- SESSION_DURATION_IN_BLOCKS: it will send the session duration in blocks.
- StakingEventType: This struct is used to test the events from the staking built-in actor
- ONE_TOKEN: one token value
- CONTRACT_EXISTENCIAL_DEPOSIT: contract existencia deposit
- ContractCommandError: Enum that will list the error that ou cacn get when you send a message to a contract.
- ContractResponse: This enum represents the state of a reponse of a contract.
- Request: requests that cacn be sent to the staking built-in actor.
- RewardAccount: the account to send the rewards from the staking built-in actor.
- ActorId32: This is the same as `ActorId` from the crate sails-rs, but you need to use this type if you will send an ActorId in the payload to a contract.
- Contract: this type is used to handle all the methods that can be used in tests from a contract.

<a id="contract_functions_sections"></a>

## 🛠️ Contract functions

vrtest has a special type called "Contract", which has very useful related functions and methods to be able to test your smart contracts:

> Note: If you send a payload to your contract, it must derive the "Encode" and "Clone" traits, and if it expects a response, the expected type must derive "Decode" to decode the response and transform it into the desired type. When sending arguments, they must be enclosed in a tuple if there is more than one; if there is only one, only the data can be sent.

### Contract methods

When you upload a contract, you'll receive a Contract type, with this you can call differents methods:

- `address`: This method will return the contract address (ActorId32).
- `new_command`: This method will return the command builder to build your new command 

> Note: In this version, to send a query, you need to transform the commando into a query.

- `new_calculate_gas`: This method will return the calculate gas builder to calculate the gas fees from a call to the contract.
- `stash_ledger`: This function will return the ledger stash of the contract, with this information you can check the amount of tokens staked of the contract, etc.
- `payee_ledger`: This function returns the payee ledger of the contract.
- `nominators`: This function returns the contract nominators.
- `free_balance`: This function returns the contracts free balance.
- `frozen_balance`: This function returns the amount of tokens that are staked, blocked, etc of the contract.
- `get_account_data`: This function will return more data about the balance of the contract.

### Builders

When you call some functions of a contract, you will receive a builder, this builders will help you to build a command, a call or even upload your contract.

- `UploadWasm`: This builder helps you to build the extrinsic to upload the contract and then upload it. You get this builder when you call the `Contract::upload_contract` related function, it contains the next methods:
    - signer: This method will set the account who will sign the extrinsic
    - salt: This method will set the salt to upload your contract (if you'll upload the same contract more than one times, you have to specify different salt values for each contract).
    - wasm: The binary of your contract
    - gas_limit: gas limit for the transaction, you can omit this method.
    - keep_alive: to keep alive the account that are uploading the contract, you can omit this method.

- `UploadSailsWasm`: This builder helps you to build the extrinsic to upload the contract that use the sails framework. You get this builder when you call the `Contract::upload_sails_contract` related function, it contains the next methods:
    - signer: This method will set the account who will sign the extrinsic
    - salt: This method will set the salt to upload your contract (if you'll upload the same contract more than one times, you have to specify different salt values for each contract).
    - app_constructor_name: This method will set the contract constructor name to init your smart contract in the runtime.
    - init_payload: is the init payload to send to the contract  to the constructor that you specify in the app_constructor_name, this is optional, but if you omit it, you need to use the unit type with turbofish.
    - wasm: The binary of your contract
    - gas_limit: gas limit for the transaction, you can omit this method.
    - keep_alive: to keep alive the account that are uploading the contract, you can omit this method.
- `CalculateGasCall`: This will help you to build the "calculate gas" extrinsic, and will returns data about the amount of tokens burned, min gas limit of gas fees to spend to send the messages, etc. You can get this builder when you call the method `new_calculate_gas`. It contains the next methods:
    - no_sails_command: This method is only to set that the message is not for a contract that implement the sails framework.
    - signer: This method will set the account who will sign the extrinsic.
    - service_name: for contracts that implement the sails framework, it set the service where you will send the message.
    - method_name: It will set the method to send the message.
    - with_value: This method will set the value that is sent along with the message, you can omit this function.
    - allow_other_panics: this functions will enable others panics, you can omit this method.
    - initial_gas: initial gas to be used in the message, this can be omitted.
    - gas_allowance: gas that can be used with the test, you can omit this function, this method can be omitted.
    - add_arg: payload to send to the contract, it need to derive Encode and Clone traits, you can set all your payload at once, Or you can add argument by argument by calling this function in order with each parameter of your smart contract's method
- `CommandCall`: This will help you to build your message that will be send to your contract, you get this builder when you call the method `new_command`. It cocntains the next methods:
    - skip_waited: When your message enter in a waited state, the method will advance from block to block until the message leaves this state. You can omit this method.
    - transform_to_query: This method will transform your command into a query, it will no change the contract state or the signer banlance. If you omit this method, it will send a normal command to your contract.
    - no_sails_command: This will transform your transaction to send the message to a contract that dont use the sails framework, if you call this method, you can omit the methods: transform_to_query, service_name and method_name.
    - signer: the id that will sign the transaction
    - service_name: This method is used when you will send a message to a contract that use the sails framework, it set the service to call from the contract.
    - method_name: This method will set the method to call in the contract, is used when you use the sails framework in your contract.
    - gas_limit: gas limit for the message.
    - keep_alive: to keep alive the account who sign the transaction, you can omit this method.
    - with_value: The value that will be send with the message.
    - max_blocks_to_wait: In case that you will wait for the response from the contract, this set the max blocks to wait for the respose, you can omit this method (it will be wait for 5 blocks).
    - add_arg: payload to send to the contract, it need to derive Encode and Clone traits, you can set all your payload at once, Or you can add argument by argument by calling this function in order with each parameter of your smart contract's method.
    - send: This method will send the command to the contract, it dont wait for the contracts response.
    - send_and_run_one_block: Same as send, but it wil go to the next block when finished.
    - send_recv: same as send, but it will go block by block to find the contract response.
    - send_check_result: same as send, but it will check if there is no errors when you send the message.   

### Upload a contract

The `Contract` type containes two related functiones that will help you to upload your smart contracts in the runtime. This functions will returns a `Builder` that will help you to "build" your contract, using or not sails.

The return value will be useful to send mesages to the contract, and check more features.

Both functions works with functional programming concepts, so, you only need to call each function in the builders to build your contracts.

When you upload a contract, the user that sign the transaction will transfer one token to the contract, because is the *contract existencial deposit*.

> Note: to be able to use this two functions you need to import the `UploadWasmT` interface to call the next methods from the builders module.

- Upload Wasm

    This related function will help you to upload the wasm from a contract, this function spect a contract that does not use sails framerk, example:

    ```rust
    use vrtest::{
        runtime::*,
        contract::{
            Contract, 
            builders::UploadWasmT
        }
    };
    use your_contract::WASM_BINARY;

    const SIGNER: u64 = 1;

    #[test]
    pub fn upload_wasm() {
        init_logger();

        new_test_ext(vec![SIGNER]).execute_with(|| {
            let contract = Contract::upload_contract()
                .signer(SIGNER)   // Who will sign the upload of the contract
                .salt("contract") // Salt to upload the contract
                .wasm(WASM_BINARY) // Contract wasm
                .upload();  // Upload your contract
        });
    }
    ```

- Upload sails contract

    This related function will upload your contract that use the sails framework, like the "upload_contract" related function, you need to specify the signer, salt, and wasm, buth with contracts that works with sails, you have to set the contract constructor name and the initial payload:

    ```rust
    use vrtest::{
        runtime::*,
        contract::{
            Contract, 
            builders::UploadWasmT
        }
    };
    use your_contract::WASM_BINARY;

    const SIGNER: u64 = 1;

    #[test]
    pub fn upload_wasm() {
        init_logger();

        new_test_ext(vec![SIGNER]).execute_with(|| {
            let contract1 = Contract::upload_sails_contract::<()>()
                .signer(SIGNER)   // Who will sign the upload of the contract
                .salt("contract") // Salt to upload the contract
                .app_constructor_name("New") // Contract constructor name
                .init_payload(())  // initial payload
                .wasm(WASM_BINARY) // Contract wasm
                .upload(); // Contract wasm

            // If your constructor dont have initial payload you can omit the
            // payload function, but you need to specify the unit type using 
            // turbofish:
            let contract2 = Contract::upload_sails_contract()
                .signer(SIGNER)   // Who will sign the upload of the contract
                .salt("contract") // Salt to upload the contract
                .app_constructor_name("New") // Contract constructor name
                .wasm(WASM_BINARY) // Contract wasm
                .upload(); // Contract wasm
        });
    }
    ```

### Calculate gas

To calculate the gas you can follow the next examples:

```rust
use vrtest::{
    runtime::*,
    contract::{
        Contract, 
        builders::UploadWasmT
    }
};
use your_contract::WASM_BINARY;

const SIGNER: u64 = 1;

#[test]
pub fn upload_wasm() {
    init_logger();

    new_test_ext(vec![SIGNER]).execute_with(|| {
        // using turbofish because payload is omitted
        let contract = Contract::upload_sails_contract::<()>()
            .signer(SIGNER)   // Who will sign the upload of the contract
            .salt("contract") // Salt to upload the contract
            .app_constructor_name("New") // Contract constructor name
            .wasm(WASM_BINARY) // Contract wasm
            .upload(); // Contract wasm

        // Get the gas fees data and more estimations.
        let gas_fees = contract.new_calculate_gas()
            .signer(SIGNER)
            .with_value(ONE_TOKEN)
            .service_name("ContractService")
            .method_name("SendValue")
            .calculate_gas();
    });
}
```

### Send a command

You ccan follow the next examples to send a message to your contract.

```rust
use vrtest::{
    runtime::*,
    types::*,
    contract::{
        Contract,
        builders::UploadWasmT
    }
};
use your_contract::WASM_BINARY;

const SIGNER: u64 = 1;

#[test]
pub fn upload_wasm() {
    init_logger();

    new_test_ext(vec![SIGNER]).execute_with(|| {
        // using turbofish because payload is omitted
        let contract = Contract::upload_sails_contract::<()>()
            .signer(SIGNER)   // Who will sign the upload of the contract
            .salt("contract") // Salt to upload the contract
            .app_constructor_name("New") // Contract constructor name
            .wasm(WASM_BINARY) // Contract wasm
            .upload(); // Contract wasm

        // Send a message to a contract that dont use sails
        let result = contract.new_command()
            .signer(SIGNER) // Set the signer
            .with_value(100 * ONE_TOKEN) // Send 100 Tokens
            .add_arg(Request::Bond {  // add this aegument in the payload
                value: 100 * ONE_TOKEN, 
                payee: RewardAccount::Program 
            })
            .no_sails_command() // Specify that the message dont use sails
            .send(); // send and dont receive a response.

        // Send a command without payload and get the response from the contract
        // The String type derive the Encode and Clone traits
        let result = contract.new_command() 
            .signer(SIGNER) // Set the signer
            .service_name("ContractService") // Set the service to call
            .method_name("SendValue") // Set the method to call
            .with_value(ONE_TOKEN) // Add one token in the message
            .send_recv::<String>(); // send and receive the 
                                    // response from the contract
            
        // Send a message to a sails contract
        let result = contract.new_command()
            .signer(SIGNER) // Set the signer
            .service_name("ContractService") // Set the service to call
            .method_name("Bond") // Set the method to call
            // Arguments are added in order to be added to the final payload.
            .add_arg(100 * ONE_TOKEN)
            .add_arg(RewardAccount::Program)
            .with_value(100 * ONE_TOKEN) // Set the value to send in the message
            .send_and_run_one_block(); // Send and go to the next block
    });
}
```
