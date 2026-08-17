use std::collections::BTreeSet;

use crate::error::{ZlangError, ZlangResult};

use super::{BytecodeModule, Capability, Instruction, Register, REGISTER_COUNT};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmLimits {
    pub max_instructions: usize,
    pub max_output_bytes: usize,
}

impl Default for VmLimits {
    fn default() -> Self {
        Self {
            max_instructions: 100_000,
            max_output_bytes: 64 * 1024,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityPolicy {
    allowed: BTreeSet<Capability>,
}

impl CapabilityPolicy {
    pub fn deny_all() -> Self {
        Self {
            allowed: BTreeSet::new(),
        }
    }

    pub fn console_only() -> Self {
        let mut allowed = BTreeSet::new();
        allowed.insert(Capability::ConsoleWrite);
        Self { allowed }
    }

    pub fn allows(&self, capability: Capability) -> bool {
        self.allowed.contains(&capability)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditOutcome {
    Allowed,
    Denied(String),
    Failed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEvent {
    pub sequence: usize,
    pub instruction_pointer: usize,
    pub instruction: &'static str,
    pub outcome: AuditOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionReport {
    pub outputs: Vec<String>,
    pub registers: Vec<i64>,
    pub instructions_executed: usize,
    pub audit: Vec<AuditEvent>,
}

pub struct ZVirtualMachine {
    policy: CapabilityPolicy,
    limits: VmLimits,
    audit: Vec<AuditEvent>,
}

impl ZVirtualMachine {
    pub fn new(policy: CapabilityPolicy, limits: VmLimits) -> Self {
        Self {
            policy,
            limits,
            audit: Vec::new(),
        }
    }

    pub fn with_console_policy() -> Self {
        Self::new(CapabilityPolicy::console_only(), VmLimits::default())
    }

    pub fn audit_log(&self) -> &[AuditEvent] {
        &self.audit
    }

    pub fn execute(&mut self, module: &BytecodeModule) -> ZlangResult<ExecutionReport> {
        self.audit.clear();
        module.validate()?;
        if module.instructions.len() > self.limits.max_instructions {
            return Err(ZlangError::ResourceLimit(format!(
                "module declares {} instructions, exceeding limit {}",
                module.instructions.len(),
                self.limits.max_instructions
            )));
        }

        let mut registers = vec![0_i64; usize::from(module.register_count)];
        let mut outputs = Vec::new();
        let mut output_bytes = 0_usize;
        let mut instruction_pointer = 0_usize;

        while instruction_pointer < module.instructions.len() {
            if instruction_pointer >= self.limits.max_instructions {
                return Err(ZlangError::ResourceLimit(format!(
                    "execution exceeded instruction limit {}",
                    self.limits.max_instructions
                )));
            }
            let instruction = &module.instructions[instruction_pointer];
            let result = self.execute_instruction(
                module,
                instruction,
                &mut registers,
                &mut outputs,
                &mut output_bytes,
            );
            match result {
                Ok(should_halt) => {
                    self.audit.push(AuditEvent {
                        sequence: self.audit.len() + 1,
                        instruction_pointer,
                        instruction: instruction.name(),
                        outcome: AuditOutcome::Allowed,
                    });
                    instruction_pointer += 1;
                    if should_halt {
                        break;
                    }
                }
                Err(error) => {
                    let outcome = match &error {
                        ZlangError::CapabilityDenied(message) => {
                            AuditOutcome::Denied(message.clone())
                        }
                        _ => AuditOutcome::Failed(error.to_string()),
                    };
                    self.audit.push(AuditEvent {
                        sequence: self.audit.len() + 1,
                        instruction_pointer,
                        instruction: instruction.name(),
                        outcome,
                    });
                    return Err(error);
                }
            }
        }

        Ok(ExecutionReport {
            outputs,
            registers,
            instructions_executed: instruction_pointer,
            audit: self.audit.clone(),
        })
    }

    fn execute_instruction(
        &self,
        module: &BytecodeModule,
        instruction: &Instruction,
        registers: &mut [i64],
        outputs: &mut Vec<String>,
        output_bytes: &mut usize,
    ) -> ZlangResult<bool> {
        match instruction {
            Instruction::Halt => Ok(true),
            Instruction::LoadImm { destination, value } => {
                set(registers, *destination, *value)?;
                Ok(false)
            }
            Instruction::Mov {
                destination,
                source,
            } => {
                let value = get(registers, *source)?;
                set(registers, *destination, value)?;
                Ok(false)
            }
            Instruction::Add {
                destination,
                left,
                right,
            } => {
                let value = get(registers, *left)?
                    .checked_add(get(registers, *right)?)
                    .ok_or_else(|| ZlangError::Runtime("integer overflow in ADD".to_string()))?;
                set(registers, *destination, value)?;
                Ok(false)
            }
            Instruction::Sub {
                destination,
                left,
                right,
            } => {
                let value = get(registers, *left)?
                    .checked_sub(get(registers, *right)?)
                    .ok_or_else(|| ZlangError::Runtime("integer overflow in SUB".to_string()))?;
                set(registers, *destination, value)?;
                Ok(false)
            }
            Instruction::Mul {
                destination,
                left,
                right,
            } => {
                let value = get(registers, *left)?
                    .checked_mul(get(registers, *right)?)
                    .ok_or_else(|| ZlangError::Runtime("integer overflow in MUL".to_string()))?;
                set(registers, *destination, value)?;
                Ok(false)
            }
            Instruction::Div {
                destination,
                left,
                right,
            } => {
                let divisor = get(registers, *right)?;
                if divisor == 0 {
                    return Err(ZlangError::Runtime("division by zero in DIV".to_string()));
                }
                let dividend = get(registers, *left)?;
                let value = dividend
                    .checked_div(divisor)
                    .ok_or_else(|| ZlangError::Runtime("integer overflow in DIV".to_string()))?;
                set(registers, *destination, value)?;
                Ok(false)
            }
            Instruction::Neg {
                destination,
                source,
            } => {
                let value = get(registers, *source)?
                    .checked_neg()
                    .ok_or_else(|| ZlangError::Runtime("integer overflow in NEG".to_string()))?;
                set(registers, *destination, value)?;
                Ok(false)
            }
            Instruction::Emit { source } => {
                self.require(module, Capability::ConsoleWrite)?;
                let output = get(registers, *source)?.to_string();
                let next_size = output_bytes.checked_add(output.len()).ok_or_else(|| {
                    ZlangError::ResourceLimit("output accounting overflow".to_string())
                })?;
                if next_size > self.limits.max_output_bytes {
                    return Err(ZlangError::ResourceLimit(format!(
                        "output exceeds {} bytes",
                        self.limits.max_output_bytes
                    )));
                }
                *output_bytes = next_size;
                outputs.push(output);
                Ok(false)
            }
        }
    }

    fn require(&self, module: &BytecodeModule, capability: Capability) -> ZlangResult<()> {
        if !module.capabilities.contains(&capability) {
            return Err(ZlangError::CapabilityDenied(format!(
                "module did not declare {capability:?}"
            )));
        }
        if !self.policy.allows(capability) {
            return Err(ZlangError::CapabilityDenied(format!(
                "runtime policy denies {capability:?}"
            )));
        }
        Ok(())
    }
}

fn get(registers: &[i64], register: Register) -> ZlangResult<i64> {
    registers
        .get(usize::from(register.0))
        .copied()
        .ok_or_else(|| {
            ZlangError::Runtime(format!(
                "register R{} is outside the physical register file of {REGISTER_COUNT} registers",
                register.0
            ))
        })
}

fn set(registers: &mut [i64], register: Register, value: i64) -> ZlangResult<()> {
    let slot = registers.get_mut(usize::from(register.0)).ok_or_else(|| {
        ZlangError::Runtime(format!(
            "register R{} is outside the physical register file of {REGISTER_COUNT} registers",
            register.0
        ))
    })?;
    *slot = value;
    Ok(())
}
