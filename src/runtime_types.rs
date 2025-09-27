use gear_core::ids::ActorId;
use core::cell::RefCell;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExecutionTraceFrame {
    pub destination: u64,
    pub source: ActorId,
    pub input: Vec<u8>,
    pub is_success: bool,
}

thread_local! {
    pub static DEBUG_EXECUTION_TRACE: RefCell<Vec<ExecutionTraceFrame>> = const { RefCell::new(Vec::new()) };
    pub static IN_TRANSACTION: RefCell<bool> = const { RefCell::new(false) };
}