use sha2::{Digest, Sha256};

use crate::error::{ZlangError, ZlangResult};

pub const BYTECODE_MAGIC: [u8; 4] = *b"ZREG";
pub const BYTECODE_VERSION: u16 = 1;
pub const REGISTER_COUNT: usize = 16;
pub const MAX_MODULE_CODE_BYTES: usize = 1_048_576;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Register(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Capability {
    ConsoleWrite = 1,
}

impl Capability {
    fn decode(value: u8) -> ZlangResult<Self> {
        match value {
            1 => Ok(Self::ConsoleWrite),
            _ => Err(ZlangError::Bytecode(format!(
                "unknown capability identifier {value}"
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Halt,
    LoadImm {
        destination: Register,
        value: i64,
    },
    Mov {
        destination: Register,
        source: Register,
    },
    Add {
        destination: Register,
        left: Register,
        right: Register,
    },
    Sub {
        destination: Register,
        left: Register,
        right: Register,
    },
    Mul {
        destination: Register,
        left: Register,
        right: Register,
    },
    Div {
        destination: Register,
        left: Register,
        right: Register,
    },
    Neg {
        destination: Register,
        source: Register,
    },
    Emit {
        source: Register,
    },
}

impl Instruction {
    fn encode(&self, output: &mut Vec<u8>) {
        match self {
            Self::Halt => output.push(0),
            Self::LoadImm { destination, value } => {
                output.push(1);
                output.push(destination.0);
                output.extend_from_slice(&value.to_le_bytes());
            }
            Self::Mov {
                destination,
                source,
            } => {
                output.extend_from_slice(&[2, destination.0, source.0]);
            }
            Self::Add {
                destination,
                left,
                right,
            } => output.extend_from_slice(&[3, destination.0, left.0, right.0]),
            Self::Sub {
                destination,
                left,
                right,
            } => output.extend_from_slice(&[4, destination.0, left.0, right.0]),
            Self::Mul {
                destination,
                left,
                right,
            } => output.extend_from_slice(&[5, destination.0, left.0, right.0]),
            Self::Div {
                destination,
                left,
                right,
            } => output.extend_from_slice(&[6, destination.0, left.0, right.0]),
            Self::Neg {
                destination,
                source,
            } => output.extend_from_slice(&[7, destination.0, source.0]),
            Self::Emit { source } => output.extend_from_slice(&[8, source.0]),
        }
    }

    fn decode(cursor: &mut Cursor<'_>) -> ZlangResult<Self> {
        let opcode = cursor.byte()?;
        let register = |cursor: &mut Cursor<'_>| cursor.byte().map(Register);
        match opcode {
            0 => Ok(Self::Halt),
            1 => Ok(Self::LoadImm {
                destination: register(cursor)?,
                value: i64::from_le_bytes(cursor.array()?),
            }),
            2 => Ok(Self::Mov {
                destination: register(cursor)?,
                source: register(cursor)?,
            }),
            3 => Ok(Self::Add {
                destination: register(cursor)?,
                left: register(cursor)?,
                right: register(cursor)?,
            }),
            4 => Ok(Self::Sub {
                destination: register(cursor)?,
                left: register(cursor)?,
                right: register(cursor)?,
            }),
            5 => Ok(Self::Mul {
                destination: register(cursor)?,
                left: register(cursor)?,
                right: register(cursor)?,
            }),
            6 => Ok(Self::Div {
                destination: register(cursor)?,
                left: register(cursor)?,
                right: register(cursor)?,
            }),
            7 => Ok(Self::Neg {
                destination: register(cursor)?,
                source: register(cursor)?,
            }),
            8 => Ok(Self::Emit {
                source: register(cursor)?,
            }),
            _ => Err(ZlangError::Bytecode(format!("unknown opcode {opcode}"))),
        }
    }

    fn registers(&self) -> Vec<Register> {
        match self {
            Self::Halt => Vec::new(),
            Self::LoadImm { destination, .. } => vec![*destination],
            Self::Mov {
                destination,
                source,
            }
            | Self::Neg {
                destination,
                source,
            } => vec![*destination, *source],
            Self::Add {
                destination,
                left,
                right,
            }
            | Self::Sub {
                destination,
                left,
                right,
            }
            | Self::Mul {
                destination,
                left,
                right,
            }
            | Self::Div {
                destination,
                left,
                right,
            } => vec![*destination, *left, *right],
            Self::Emit { source } => vec![*source],
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Halt => "HALT",
            Self::LoadImm { .. } => "LOAD_IMM",
            Self::Mov { .. } => "MOV",
            Self::Add { .. } => "ADD",
            Self::Sub { .. } => "SUB",
            Self::Mul { .. } => "MUL",
            Self::Div { .. } => "DIV",
            Self::Neg { .. } => "NEG",
            Self::Emit { .. } => "EMIT",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BytecodeModule {
    pub version: u16,
    pub register_count: u8,
    pub capabilities: Vec<Capability>,
    pub instructions: Vec<Instruction>,
}

impl BytecodeModule {
    pub fn new(
        version: u16,
        register_count: u8,
        capabilities: Vec<Capability>,
        instructions: Vec<Instruction>,
    ) -> ZlangResult<Self> {
        let module = Self {
            version,
            register_count,
            capabilities,
            instructions,
        };
        module.validate()?;
        Ok(module)
    }

    pub fn validate(&self) -> ZlangResult<()> {
        if self.version != BYTECODE_VERSION {
            return Err(ZlangError::Bytecode(format!(
                "unsupported bytecode version {}; runtime supports {BYTECODE_VERSION}",
                self.version
            )));
        }
        if self.register_count == 0 || usize::from(self.register_count) > REGISTER_COUNT {
            return Err(ZlangError::Bytecode(format!(
                "register count must be between 1 and {REGISTER_COUNT}"
            )));
        }
        if self.instructions.is_empty() {
            return Err(ZlangError::Bytecode(
                "module has no instructions".to_string(),
            ));
        }
        if !matches!(self.instructions.last(), Some(Instruction::Halt)) {
            return Err(ZlangError::Bytecode(
                "module must end with a HALT instruction".to_string(),
            ));
        }
        if self.instructions[..self.instructions.len() - 1]
            .iter()
            .any(|instruction| matches!(instruction, Instruction::Halt))
        {
            return Err(ZlangError::Bytecode(
                "HALT is only valid as the last instruction".to_string(),
            ));
        }
        for capability in self.capabilities.windows(2) {
            if capability[0] >= capability[1] {
                return Err(ZlangError::Bytecode(
                    "capabilities must be sorted and unique".to_string(),
                ));
            }
        }
        for instruction in &self.instructions {
            for register in instruction.registers() {
                if register.0 >= self.register_count {
                    return Err(ZlangError::Bytecode(format!(
                        "{} references R{} outside configured register file R0..R{}",
                        instruction.name(),
                        register.0,
                        self.register_count - 1
                    )));
                }
            }
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> ZlangResult<Vec<u8>> {
        self.validate()?;
        let mut code = Vec::new();
        for instruction in &self.instructions {
            instruction.encode(&mut code);
        }
        if code.len() > MAX_MODULE_CODE_BYTES {
            return Err(ZlangError::Bytecode(format!(
                "code section exceeds {MAX_MODULE_CODE_BYTES} bytes"
            )));
        }

        let mut output = Vec::with_capacity(12 + self.capabilities.len() + code.len() + 32);
        output.extend_from_slice(&BYTECODE_MAGIC);
        output.extend_from_slice(&self.version.to_le_bytes());
        output.push(self.register_count);
        output.push(self.capabilities.len() as u8);
        output.extend_from_slice(&(code.len() as u32).to_le_bytes());
        output.extend(self.capabilities.iter().map(|capability| *capability as u8));
        output.extend_from_slice(&code);
        let checksum = Sha256::digest(&output);
        output.extend_from_slice(&checksum);
        Ok(output)
    }

    pub fn from_bytes(bytes: &[u8]) -> ZlangResult<Self> {
        const HEADER_LEN: usize = 12;
        const CHECKSUM_LEN: usize = 32;
        if bytes.len() < HEADER_LEN + CHECKSUM_LEN {
            return Err(ZlangError::Bytecode(
                "module is smaller than header and checksum".to_string(),
            ));
        }
        let payload_len = bytes.len() - CHECKSUM_LEN;
        let expected = Sha256::digest(&bytes[..payload_len]);
        if expected.as_slice() != &bytes[payload_len..] {
            return Err(ZlangError::Bytecode(
                "module checksum verification failed".to_string(),
            ));
        }

        let mut cursor = Cursor::new(&bytes[..payload_len]);
        let magic: [u8; 4] = cursor.array()?;
        if magic != BYTECODE_MAGIC {
            return Err(ZlangError::Bytecode(
                "invalid bytecode magic; expected ZREG".to_string(),
            ));
        }
        let version = u16::from_le_bytes(cursor.array()?);
        let register_count = cursor.byte()?;
        let capability_count = usize::from(cursor.byte()?);
        let code_len = u32::from_le_bytes(cursor.array()?) as usize;
        if code_len > MAX_MODULE_CODE_BYTES {
            return Err(ZlangError::Bytecode(format!(
                "code section exceeds {MAX_MODULE_CODE_BYTES} bytes"
            )));
        }
        let mut capabilities = Vec::with_capacity(capability_count);
        for _ in 0..capability_count {
            capabilities.push(Capability::decode(cursor.byte()?)?);
        }
        let code = cursor.take(code_len)?;
        if cursor.remaining() != 0 {
            return Err(ZlangError::Bytecode(
                "module has trailing bytes before checksum".to_string(),
            ));
        }

        let mut code_cursor = Cursor::new(code);
        let mut instructions = Vec::new();
        while code_cursor.remaining() > 0 {
            instructions.push(Instruction::decode(&mut code_cursor)?);
        }
        Self::new(version, register_count, capabilities, instructions)
    }
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.position)
    }

    fn take(&mut self, count: usize) -> ZlangResult<&'a [u8]> {
        let end = self
            .position
            .checked_add(count)
            .ok_or_else(|| ZlangError::Bytecode("module offset overflow".to_string()))?;
        let slice = self
            .bytes
            .get(self.position..end)
            .ok_or_else(|| ZlangError::Bytecode("unexpected end of module".to_string()))?;
        self.position = end;
        Ok(slice)
    }

    fn byte(&mut self) -> ZlangResult<u8> {
        Ok(self.take(1)?[0])
    }

    fn array<const N: usize>(&mut self) -> ZlangResult<[u8; N]> {
        self.take(N)?
            .try_into()
            .map_err(|_| ZlangError::Bytecode("invalid fixed-width bytecode field".to_string()))
    }
}
