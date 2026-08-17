mod bytecode;
mod runtime;

pub use bytecode::{
    BytecodeModule, Capability, Instruction, Register, BYTECODE_MAGIC, BYTECODE_VERSION,
    REGISTER_COUNT,
};
pub use runtime::{
    AuditEvent, AuditOutcome, CapabilityPolicy, ExecutionReport, VmLimits, ZVirtualMachine,
};
