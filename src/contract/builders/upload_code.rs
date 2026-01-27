use frame_support::assert_ok;
use gear_core::ids::CodeId;
use crate::{
    mock::{
        Gear,
        RuntimeOrigin
    },
};
use sp_core::blake2_256;

#[derive(Default)]
pub struct UploadCode {
    signer: Option<u64>,
    wasm: Option<Vec<u8>>
}

impl UploadCode {
    pub fn signer(mut self, signer: u64) -> Self {
        self.signer = Some(signer);

        self
    }

    pub fn wasm(mut self, wasm: &[u8]) -> Self {
        self.wasm = Some(wasm.to_vec());

        self
    }

    pub fn upload(self) -> CodeId {
        if self.signer.is_none() {
            panic!("Signer is not set!");
        }

        if self.wasm.is_none() {
            panic!("Wasm not set!!");
        }
        
        let wasm = self.wasm.unwrap();

        assert_ok!(
            Gear::upload_code(
                RuntimeOrigin::signed(self.signer.unwrap()), 
                wasm.clone()
            )
        );

        let code_id = CodeId::from(blake2_256(&wasm));

        code_id
    }
}