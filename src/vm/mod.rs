use crate::compiler::{ZLB2_EMIT, ZLB2_HALT, ZLB2_HEADER};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmError {
    message: String,
}

impl VmError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for VmError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for VmError {}

#[derive(Default)]
pub struct ZVirtualMachine;

impl ZVirtualMachine {
    pub fn new() -> Self {
        Self
    }

    /// Execute the same bootstrap profile accepted by ZDOS x86_64.
    pub fn execute(&mut self, bytecode: &[u8]) -> Result<String, VmError> {
        if bytecode.len() < ZLB2_HEADER.len() || bytecode[..6] != ZLB2_HEADER {
            return Err(VmError::new("header ZLB2 non valido"));
        }

        let mut output = String::new();
        let mut offset = ZLB2_HEADER.len();
        while offset < bytecode.len() {
            if bytecode.len() - offset < 3 {
                return Err(VmError::new("header del record ZLB2 troncato"));
            }
            let opcode = bytecode[offset];
            let payload_len =
                u16::from_le_bytes([bytecode[offset + 1], bytecode[offset + 2]]) as usize;
            let payload_start = offset + 3;
            let payload_end = payload_start
                .checked_add(payload_len)
                .ok_or_else(|| VmError::new("overflow della lunghezza del record ZLB2"))?;
            if payload_end > bytecode.len() {
                return Err(VmError::new("payload del record ZLB2 oltre il buffer"));
            }

            match opcode {
                ZLB2_EMIT => {
                    let text = std::str::from_utf8(&bytecode[payload_start..payload_end])
                        .map_err(|_| VmError::new("payload EMIT non UTF-8"))?;
                    output.push_str(text);
                    output.push('\n');
                }
                ZLB2_HALT => {
                    if payload_len != 0 || payload_end != bytecode.len() {
                        return Err(VmError::new("HALT ZLB2 non terminale o con payload"));
                    }
                    return Ok(output);
                }
                _ => {
                    return Err(VmError::new(format!(
                        "opcode ZLB2 non supportato dal profilo bootstrap: 0x{opcode:02x}"
                    )));
                }
            }
            offset = payload_end;
        }

        Err(VmError::new("HALT ZLB2 mancante"))
    }
}

#[cfg(test)]
mod tests {
    use super::ZVirtualMachine;
    use crate::compiler::Compiler;

    #[test]
    fn executes_emit_bytecode() {
        let bytecode = Compiler::new()
            .compile("emit ciao\n")
            .expect("valid source");
        let output = ZVirtualMachine::new()
            .execute(&bytecode)
            .expect("valid bytecode");
        assert_eq!(output, "ciao\n");
    }

    #[test]
    fn rejects_unimplemented_opcode() {
        let bytecode = b"ZLB2\x02\x05\x02\x00\x00\xff\x00\x00";
        let error = ZVirtualMachine::new()
            .execute(bytecode)
            .expect_err("must fail closed");
        assert!(error.to_string().contains("non supportato"));
    }
}
